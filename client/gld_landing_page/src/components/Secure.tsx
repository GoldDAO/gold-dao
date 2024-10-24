/* eslint-disable react/no-unescaped-entities */
'use client'

const Secure = () => {
  return (
    <section className='flex flex-col items-center justify-center gap-4 w-full mb-[96px] px-2 md:px-10 bg-[#FAF9F8]'>
      {/* Titre */}
      <p className='font-bold text-3xl text-center'>
        Secure, audited, and regulated
      </p>

      {/* Contenu */}
      <div className='space-y-4 text-center w-full md:w-1/2 2xl:w-[51%] 3xl:w-1/2 py-8 text-[#000000CC]'>
        {/* Sous-titre */}
        <p className='font-bold'>Peace of mind for your investment</p>

        {/* Description 1 avec lien */}
        <p>
          To ensure maximum safety, security and transparency, Gold DAO’s gold
          assets are audited every four months by KPMG, which issues an
          independent audit report verifying that the physical gold bullion is
          present and securely stored. These audit reports can be viewed at{' '}
          <a
            href='https://yumi.io/gold/audit'
            target='_blank'
            rel='noopener noreferrer'
            className=' underline hover:opacity-80'
          >
            this link
          </a>
          .
        </p>

        {/* Description 2 avec lien */}
        <p>
          All gold assets are sourced from{' '}
          <a
            href='https://metalor.com/'
            target='_blank'
            rel='noopener noreferrer'
            className=' underline hover:opacity-80'
          >
            Metalor®
          </a>
          , one of the world's most trusted gold refiners, and securely stored
          in high-security Swiss vaults provided by{' '}
          <a
            href='https://ch.loomis.com/de-ch'
            target='_blank'
            rel='noopener noreferrer'
            className='underline hover:opacity-80'
          >
            Loomis
          </a>
          , giving you peace of mind that your investment is fully protected.
        </p>
      </div>
    </section>
  )
}

export default Secure
