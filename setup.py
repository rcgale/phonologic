import setuptools

setuptools.setup(
    name='phonologic',
    version='0.2.0a2',
    author='Robert Gale',
    author_email='galer@ohsu.edu',
    packages=setuptools.find_packages(),
    url='https://github.com/rcgale/phonologic',
    description='',
    install_requires=[
        'regex',
        'tqdm',
    ],
    include_package_data=True,
    package_data={
        '': [
            '**/*.py',
            '**/*.phl',
            '**/*.js',
            '**/*.json',
            '**/*.html',
            '**/*.css',
        ]
    },
    entry_points={
        'console_scripts': [
            'phonologic-viewer = phonologic.viewer:main',
        ],
    },
)
