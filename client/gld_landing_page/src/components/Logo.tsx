// The Gold DAO mark, inline so it needs no image request. The id keeps the
// gradient references apart when the mark is on the page more than once.
export default function Logo ({
  id,
  className
}: {
  id: string
  className?: string
}) {
  const silver = `${id}-silver`
  const gold = `${id}-gold`

  return (
    <svg
      viewBox='0 0 358 526'
      fill='none'
      xmlns='http://www.w3.org/2000/svg'
      aria-hidden='true'
      className={className}
    >
      <path
        d='M7.25039 80.8007L163.887 3.57932C173.68 -1.27056 185.168 -1.17639 194.913 3.76766L338.226 76.4688C350.373 82.6371 358 95.0678 358 108.676V196.115C358 211.559 349.29 225.732 335.543 232.748L192.277 305.402C183.661 309.781 173.397 303.518 173.397 293.819V267.168C173.397 262.271 176.175 257.751 180.554 255.585L295.948 197.48C300.326 195.267 303.057 190.841 303.104 185.944L303.434 121.672C303.434 116.727 300.656 112.207 296.277 109.994L184.791 54.2441C181.166 52.4077 176.881 52.4077 173.209 54.2441L18.738 130.853C10.1223 135.138 0 128.876 0 119.223V92.431C0 87.4869 2.82483 82.9667 7.25039 80.8007Z'
        fill={`url(#${silver})`}
      />
      <path
        d='M173.303 128.027L15.8661 206.991C6.16754 211.888 0 221.823 0 232.7V420.904C0 431.263 5.7909 440.727 14.9716 445.483L164.123 522.421C173.492 527.224 184.555 527.271 193.972 522.516L341.851 447.319C351.785 442.281 358 432.063 358 420.951V281.999C358 269.145 344.488 260.763 332.953 266.461L184.744 340.057C178.058 343.4 173.821 350.227 173.821 357.667V381.54C173.821 390.627 183.378 396.513 191.476 392.464L288.038 344.153C295.1 340.622 303.434 345.754 303.434 353.664V400.892C303.434 406.401 300.326 411.486 295.43 413.982L187.757 468.932C182.249 471.757 175.704 471.757 170.149 468.932L64.4532 415.347C58.3327 412.24 54.425 405.93 54.425 399.056V252.382C54.425 247.579 57.1086 243.2 61.393 241.034L176.787 182.742C180.977 180.623 183.614 176.338 183.567 171.629L183.284 134.196C183.284 129.063 177.823 125.72 173.209 128.027H173.303Z'
        fill={`url(#${gold})`}
      />
      <defs>
        <linearGradient
          id={silver}
          x1='-273.726'
          y1='602.845'
          x2='555.886'
          y2='-226.619'
          gradientUnits='userSpaceOnUse'
        >
          <stop />
          <stop offset='0.06' stopColor='#232323' />
          <stop offset='0.1' stopColor='#4F4F4F' />
          <stop offset='0.16' stopColor='#939393' />
          <stop offset='0.19' stopColor='#AEAEAE' />
          <stop offset='0.24' stopColor='#A4A4A4' />
          <stop offset='0.34' stopColor='#898989' />
          <stop offset='0.35' stopColor='#878787' />
          <stop offset='0.36' stopColor='#7B7B7B' />
          <stop offset='0.42' stopColor='#454545' />
          <stop offset='0.44' stopColor='#303030' />
          <stop offset='0.46' stopColor='#3F3F3F' />
          <stop offset='0.49' stopColor='#686868' />
          <stop offset='0.53' stopColor='#B0B0B0' />
          <stop offset='0.65' stopColor='#AFAFAF' />
          <stop offset='0.67' stopColor='#B6B6B6' />
          <stop offset='0.7' stopColor='#C9C9C9' />
          <stop offset='0.74' stopColor='#E9E9E9' />
          <stop offset='0.75' stopColor='#EDEDED' />
          <stop offset='0.83' stopColor='#EEEEEE' />
          <stop offset='0.86' stopColor='#C9C9C9' />
          <stop offset='0.9' stopColor='#939393' />
          <stop offset='0.93' stopColor='#7E7E7E' />
        </linearGradient>
        <linearGradient
          id={gold}
          x1='-177.305'
          y1='683.268'
          x2='622.878'
          y2='-116.818'
          gradientUnits='userSpaceOnUse'
        >
          <stop stopColor='#5E4C14' />
          <stop offset='0.06' stopColor='#7C6517' />
          <stop offset='0.19' stopColor='#BF9D43' />
          <stop offset='0.26' stopColor='#DAB355' />
          <stop offset='0.28' stopColor='#D0AA4E' />
          <stop offset='0.31' stopColor='#B5933E' />
          <stop offset='0.34' stopColor='#9E7F30' />
          <stop offset='0.42' stopColor='#72561A' />
          <stop offset='0.46' stopColor='#5D4310' />
          <stop offset='0.49' stopColor='#6C4F14' />
          <stop offset='0.55' stopColor='#95711E' />
          <stop offset='0.59' stopColor='#B28926' />
          <stop offset='0.62' stopColor='#BF9734' />
          <stop offset='0.65' stopColor='#C59E3B' />
          <stop offset='0.66' stopColor='#C8A342' />
          <stop offset='0.69' stopColor='#D2B455' />
          <stop offset='0.71' stopColor='#E3CF75' />
          <stop offset='0.73' stopColor='#F7F09C' />
          <stop offset='0.78' stopColor='#F8F19F' />
          <stop offset='0.82' stopColor='#E5D37E' />
          <stop offset='0.89' stopColor='#C6A348' />
          <stop offset='0.93' stopColor='#BB9133' />
        </linearGradient>
      </defs>
    </svg>
  )
}
