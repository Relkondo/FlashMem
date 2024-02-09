#!/bin/bash

# The path to the folder containing the libraries
P_LIBS="$1"
touch bundle_libs_to_add.txt
echo "Linking your bundled brew dependencies for distribution!"
echo " THIS I A DRY-RUN. UNCOMMENT INSIDE TO RUN FOR REAL"


# Iterate through the first-level folders in P_LIBS
for LIB_DIR in "$P_LIBS"/*; do
  if [ -d "$LIB_DIR" ]; then
    LIB_NAME=$(basename "$LIB_DIR")

    # Find and process all .dylib files within each LIB_DIR
    find "$LIB_DIR" \( -type f -name "*.so" -o -name "*.dylib" -o -name "*.dll" \) -o \( -type f -perm +111 \) | while read -r BUNDLE_ELEMENT; do
       echo "#### Processing $(basename "$BUNDLE_ELEMENT") in $LIB_NAME ####"
       echo ""

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

          # echo "Setting ID for $DEP_NEW_PATH to $DEP_RLTV_PATH"
          # Set the install name for the dependency itself
          # install_name_tool -id "$DEP_RLTV_PATH" "$DEP_NEW_PATH"

          # echo "Changing dependency path in $BUNDLE_ELEMENT from $DEP_ABS_PATH to $DEP_RLTV_PATH"
          # Update the dependency path in the BUNDLE_ELEMENT to the new relative path
          # install_name_tool -change "$DEP_ABS_PATH" "$DEP_RLTV_PATH" "$BUNDLE_ELEMENT"
        else
          echo "#### Processing $(basename "$BUNDLE_ELEMENT") in $LIB_NAME ####"
          echo "Warning: Could not find new path for dependency $DEP in the bundle"
          LIB_NAME=$(echo "$DEP_ABS_PATH" | awk -F'/' '{print $5}')
          echo "Lib: $LIB_NAME"
          echo "$LIB_NAME" >> bundle_libs_to_add.txt
          echo "Present Path: $DEP_ABS_PATH"
        fi
        echo ""
      done
       echo "-------------------------------"
       echo ""
    done
  fi
done
echo "The following brew dependencies were not found, please add them if you think your users might not have them:"
sort -u bundle_libs_to_add.txt -o bundle_libs_to_add.txt
cat bundle_libs_to_add.txt
rm bundle_libs_to_add.txt

echo "DEACTIVATED: THIS WAS A DRY-RUN. UNCOMMENT INSIDE TO RUN FOR REAL"