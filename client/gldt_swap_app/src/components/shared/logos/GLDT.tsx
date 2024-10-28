export const LogoGLDT = ({ className }: { className?: string }) => {
  const light = "/gldt_logo.svg";
  return (
    <img
      src={light}
      alt="GLDT Logo"
      className={className ? className : "w-10 h-10"}
    />
  );
};
