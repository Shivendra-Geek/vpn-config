// pages/index.tsx
"use client";
import  {motion}  from 'framer-motion';
import Head from 'next/head';
import Link from 'next/link';

export default function Welcome() {
  return (
    <>
      <Head>
        <title>Welcome | YourApp</title>
      </Head>

      <main className="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-100 via-white to-blue-200 px-6">
        <div className="text-center max-w-2xl">
          <motion.h1
            className="text-5xl md:text-6xl font-bold text-blue-700 mb-6"
            initial={{ opacity: 0, y: -50 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6 }}
          >
            Welcome to <span className="text-blue-500">YourApp</span>
          </motion.h1>

          <motion.p
            className="text-lg text-gray-700 mb-8"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ delay: 0.3 }}
          >
            Simplify your workflow and boost productivity with our intuitive platform.
          </motion.p>

          <motion.div
            className="flex justify-center gap-4"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ delay: 0.5 }}
          >
            <Link className="px-6 py-3 bg-blue-600 text-white rounded-full hover:bg-blue-700 transition-all duration-300" href="/signup">
                Get Started
            </Link>
            <Link className="px-6 py-3 border-2 border-blue-600 text-blue-600 rounded-full hover:bg-blue-50 transition-all duration-300" href="/login">
                Sign In
            </Link>
          </motion.div>

          {/* Optional Illustration */}
          <motion.img
            src="/illustration.svg"
            alt="Welcome Illustration"
            className="mx-auto mt-10 w-2/3 md:w-1/2"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ delay: 0.8 }}
          />
        </div>
      </main>
    </>
  );
}
