import { Layout } from 'antd';
import { useRecoilValue } from 'recoil';
import Head from 'next/head';
import { FC, useEffect, useRef } from 'react';
import { useLockBodyScroll } from 'react-use';
import dynamic from 'next/dynamic';
import {
  ClientConfigStore,
  isChatAvailableSelector,
  ClientConfigStateAtom,
  fatalErrorStateAtom,
} from '../stores/ClientConfigStore';
import { ClientConfig } from '../../interfaces/client-config.model';

// lazy-loaded components
const fatalErrorStateModal = dynamic(
   () => import('../modals/FatalErrorStateModal').then(
      mod => mod.FatalErrorStateModal,
   ),
   {
      loading: () => <div>Loading...</div>,
      ssr: false,
   }
);

export const Main: FC = () => {
   const clientConfig = useRecoilValue<ClientConfig>(ClientConfigStateAtom);

   const layoutRef = useRef<HTMLDivElement>(null);
   const isProduction = process.env.NODE_ENV === 'production';

   return (
      <>
         <Head>
            <link rel='apple-touch-icon' sizes='57x57' href='/img/favicon/apple-icon-57x57.png' />
            <link rel='apple-touch-icon' sizes='60x60' href='/img/favicon/apple-icon-60x60.png' />
            <link rel='apple-touch-icon' sizes='72x72' href='/img/favicon/apple-icon-72x72.png' />
            <link rel='apple-touch-icon' sizes='76x76' href='/img/favicon/apple-icon-76x76.png' />
            <link rel='apple-touch-icon' sizes='114x114' href='/img/favicon/apple-icon-114x114.png' />
            <link rel='apple-touch-icon' sizes='120x120' href='/img/favicon/apple-icon-120x120.png' />
            <link rel='apple-touch-icon' sizes='144x144' href='/img/favicon/apple-icon-144x144.png' />
            <link rel='apple-touch-icon' sizes='152x152' href='/img/favicon/apple-icon-152x152.png' />
            <link rel='apple-touch-icon' sizes='180x180' href='/img/favicon/apple-icon-180x180.png' />

            <link rel='manifest' href='/manifest.json' />
            <link href='/api/auth/provider/indieauth' />
            <meta name='msapplication-TileColor' content='#ffffff' />
            <meta name='msapplication-TileImage' content='/img/favicon/ms-icon-144x144.png' />
            <meta name='theme-color' content='#ffffff' />

            <style>{customStyles}</style>
            <base target='_blank' />
         </Head>

         {isProduction ? (
            <Head>
               {name ? <title>{name}</title> : <title>{'{{.Name}}'}</title>}
               <meta name='description' content='{{.Summary}}' />

               <meta property='og:title' content='{{.Name}}' />
               <meta property='og:site_name' content='{{.Name}}' />
               <meta property='og:url' content='{{.RequestedURL}}' />
               <meta property='og:description' content='{{.Summary}}' />
               <meta property='og:type' content='video.other' />
               <meta property='video:tag' content='{{.TagsString}}' />

               <meta property='og:image' content='{{.RequestedURL}}{{.Thumbnail}}' />
               <meta property='og:image:url' content='{{.RequestedURL}}{{.Thumbnail}}' />
               <meta property='og:image:alt' content='{{.RequestedURL}}{{.Image}}' />

               <meta property='og:video' content='{{.RequestedURL}}/embed/video' />
               <meta property='og:video:secure_url' content='{{.RequestedURL}}/embed/video' />
               <meta property='og:video:height' content='315' />
               <meta property='og:video:width' content='560' />
               <meta property='og:video:type' content='text/html' />
               <meta property='og:video:actor' content='{{.Name}}' />

               <meta property='twitter:title' content='{{.Name}}' />
               <meta property='twitter:url' content='{{.RequestedURL}}' />
               <meta property='twitter:description' content='{{.Summary}}' />
               <meta property='twitter:image' content='{{.Image}}' />
               <meta property='twitter:card' content='player' />
               <meta property='twitter:player' content='{{.RequestedURL}}/embed/video' />
               <meta property='twitter:player:width' content='560' />
               <meta property='twitter:player:height' content='315' />
            </Head>
         ) : (
            <Head>
               <title>{name}</title>
            </Head>
         )}

         <ClientConfigStore />
         <TitleNotifier name={name} />
         <Theme />
         <Layout ref={layoutRef} style={{ minHeight: '100vh' }}>
            <Header name={title || name} chatAvailable={isChatAvailable} chatDisabled={chatDisabled} />
            <Content />
            {fatalError && (
               <FatalErrorStateModal title={fatalError.title} message={fatalError.message} />
            )}
         </Layout>
      </>
   );
};