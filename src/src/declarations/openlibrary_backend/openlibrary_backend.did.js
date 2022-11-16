export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'createAndInstall' : IDL.Func([], [IDL.Text], []),
    'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
