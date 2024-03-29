def Settings( **kwargs ):
  if kwargs[ 'language' ] == 'rust':
    return {
      'ls': {
          'diagnostics': {
              'enable': True,
              'disabled': ["unresolved-proc-macro", "unresolved-macro-call", "unresolved-import"]
          },
          'updates': {
              'channel': 'nightly'
          },
        }
      }

