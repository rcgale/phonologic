import setuptools

setuptools.setup(
    name='phonologic',
    version='0.0.1',
    author='Robert Gale',
    author_email='galer@ohsu.edu',
    packages=[
        'phonologic',
        'phonologic._error_analysis',
        'phonologic._file_parsing',
        'phonologic.systems',
    ],
    url='https://github.com/rcgale/phonologic',
    description='',
    install_requires=[
        'regex',
    ],
    include_package_data=True,
    package_data={
        'phonologic': [
            '**/*.py',
            '**/*.phl',
            '**/*.json',
        ]
    },
)
