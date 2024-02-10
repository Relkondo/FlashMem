#!/bin/bash

# The path to the folder containing the libraries
PROFILE=$1
if [ "$PROFILE" != "debug" ] && [ "$PROFILE" != "release" ]; then
    echo "You have to input either debug or release as argument"
    exit 9
fi

echo "Linking your bundled brew dependencies for distribution!"
echo ""

echo "Ensuring permissions are set on libs..."
chmod -R u+rw src-tauri/libs

echo "Building..."
if [ "$PROFILE" == "release" ]; then
  cargo tauri build
else
  cargo tauri dev
fi

echo "Entering src-tauri..."
cd src-tauri || exit

echo "Reading tauri.conf.json..."
VOL_NAME=$(jq -r '.package.productName' tauri.conf.json)
VERSION=$(jq -r '.package.version' tauri.conf.json)
LICENSE_FILE=$(jq -r '.tauri.bundle.macOS.license' tauri.conf.json)

echo "Copying libs inside build..."
cp -R "libs" "target/$PROFILE/"
if [ "$PROFILE" == "release" ]; then
  cp -R "libs" "target/release/bundle/macos/$VOL_NAME.app/Contents/Resources/"
fi

echo "Setting proper @rpath..."

function process_one_element() {
  BUNDLE_ELEMENT=$1
  LIB_NAME=$2
  P_LIBS=$3
  echo "#### Processing $(basename "$BUNDLE_ELEMENT") in $LIB_NAME ####"

  # List dependencies using otool and process ones with specific paths
  otool -L "$BUNDLE_ELEMENT" | grep -E '(/usr/local/(opt|Cellar))' | awk '{print $1}' | while read -r DEP_ABS_PATH; do
    # Extract the filename of the dependency
    DEP=$(basename "$DEP_ABS_PATH")

    # Find the new path of DEP within P_LIBS
    DEP_NEW_PATH=$(find "$P_LIBS" -name "$DEP")

    # Ensure DEP_NEW_PATH is found and not empty
    if [ -n "$DEP_NEW_PATH" ]; then
      # Remove P_LIBS from the start of DEP_NEW_PATH to get DEP_PATH_END
      DEP_PATH_END="${DEP_NEW_PATH#$P_LIBS}"

      # Construct DEP_RLTV_PATH by appending DEP_PATH_END to @rpath
      DEP_RLTV_PATH="@rpath$DEP_PATH_END"

       echo "Setting ID for $DEP_NEW_PATH to $DEP_RLTV_PATH"
       install_name_tool -id "$DEP_RLTV_PATH" "$DEP_NEW_PATH"

       echo "Changing dependency path in $BUNDLE_ELEMENT from $DEP_ABS_PATH to $DEP_RLTV_PATH"
       install_name_tool -change "$DEP_ABS_PATH" "$DEP_RLTV_PATH" "$BUNDLE_ELEMENT"
    else
      echo "Warning: Could not find new path for dependency $DEP in the lib $P_LIBS"
      LIB_NAME=$(echo "$DEP_ABS_PATH" | awk -F'/' '{print $5}')
      echo "Lib: $LIB_NAME"
      echo "$LIB_NAME" >> bundle_libs_to_add.txt
      echo "Present Path: $DEP_ABS_PATH"
    fi
    echo ""
  done
  echo "-------------------------------"
  echo ""
}

function process_one_exec() {
  P_BUNDLE=$1
  P_BUNDLE_LIBS=$2
  P_BUNDLE_EXEC=$3
  R_PATH=$4
  echo "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++"
  echo "STARTED: $P_BUNDLE"
  echo "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++"
  echo ""
  # Iterate through the first-level folders in P_LIBS
  P_LIBS="$P_BUNDLE$P_BUNDLE_LIBS"
  P_EXECUTABLE="$P_BUNDLE$P_BUNDLE_EXEC"
  for LIB_DIR in "$P_LIBS"/*; do
    if [ -d "$LIB_DIR" ]; then
      LIB_NAME=$(basename "$LIB_DIR")
      # Find and process all .dylib files within each LIB_DIR
      find "$LIB_DIR" \( -type f -name "*.so" -o -name "*.dylib" -o -name "*.dll" \) -o \( -type f -perm +111 \) | while read -r BUNDLE_ELEMENT; do
        process_one_element "$BUNDLE_ELEMENT" "$LIB_NAME" "$P_LIBS"
      done
    fi
  done
  # Execute the same process for the executable
  process_one_element "$P_EXECUTABLE" "$P_BUNDLE (exec)" "$P_LIBS"
  echo ""
  echo "Adding rpath $R_PATH to $P_EXECUTABLE..."
  install_name_tool -add_rpath "$R_PATH" "$P_EXECUTABLE"
  echo ""
  echo "--- FINISHED: $P_BUNDLE ---"
  echo ""
}

function rebundle_dmg() {
  echo "Rebundling DMG..."
  PWD=$(pwd)
  if [ -n "$LICENSE_FILE" ] && [ "$LICENSE_FILE" != "null" ]; then
    ARGS+=("--eula" "$PWD/$LICENSE_FILE")
  fi
  echo "Entering bundle folder..."
  cd target/"$PROFILE"/bundle/macos || exit
  ARGS=("--volname" "$VOL_NAME"
  "--icon" "$VOL_NAME.app" "180" "170"
  "--app-drop-link" "480" "170"
  "--window-size" "660" "440"
  "--hide-extension" "$VOL_NAME.app"
  "--volicon" "../dmg/icon.icns"
  )
  DMG_NAME="$VOL_NAME"_"$VERSION"_x64.dmg
  ARGS+=("$DMG_NAME" "$VOL_NAME.app")
  echo "Executing: bundle_dmg.sh ${ARGS[*]}"
  ../dmg/bundle_dmg.sh "${ARGS[@]}"
  mv "$DMG_NAME" ../dmg/
  echo "Exiting bundle folder..."
  cd ../../../..
}

touch bundle_libs_to_add.txt
process_one_exec "target/$PROFILE/" "libs" "$VOL_NAME" "@executable_path/libs"
if [ "$PROFILE" == "release" ]; then
  process_one_exec "target/release/bundle/macos/$VOL_NAME.app/" "Contents/Resources/libs" "Contents/MacOS/$VOL_NAME" "@executable_path/../Resources/libs"
  rebundle_dmg
fi

sort -u bundle_libs_to_add.txt -o bundle_libs_to_add.txt
LIBS_TO_ADD=$(<bundle_libs_to_add.txt)
if [ -n "$LIBS_TO_ADD" ]; then
  echo ""
  echo "The following brew dependencies were not found, please add them if you think your users might not have them:"
  echo "$LIBS_TO_ADD"
  echo ""
fi
rm bundle_libs_to_add.txt
echo "Exiting src-tauri..."
cd ..
