<script lang="ts">
	import { initializeApp } from "firebase/app";
	// import { getAnalytics } from "firebase/analytics";
	// import {} from 'firebase/firestore';

	import { getAuth, createUserWithEmailAndPassword } from 'firebase/auth';
	const firebaseConfig = {
		apiKey: "AIzaSyBCgq-qsmF6A4LCsWDO9YgAOgVcFXaDXNw",
		authDomain: "flashsub-1b456.firebaseapp.com",
		projectId: "flashsub-1b456",
		storageBucket: "flashsub-1b456.appspot.com",
		messagingSenderId: "368028589856",
		appId: "1:368028589856:web:d3e015ef2a6e9b78b40145",
		measurementId: "G-82XNXVVPS9"
	};

	initializeApp(firebaseConfig);
	// const analytics = getAnalytics(firebase_app);
	let auth = getAuth();

	let message = "Register";

	function register() {
		let email = "samuel.coron@outlook.fr";
		let password = "testpassword1234";
		createUserWithEmailAndPassword(auth, email, password)
			.then((userCredential) => {
				const user = userCredential.user;
				console.log(user);
				message = "Registered !!";
			})
			.catch((error) => {
				const errorCode = error.code;
				const errorMessage = error.message;
				console.log(errorMessage);
			});
	}

</script>

<button on:click={register} class="bg-dark-green text-white p-2 rounded-md">{message}</button>
