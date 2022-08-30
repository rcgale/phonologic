import setuptools

setuptools.setup(
    name='phonologic',
    version='0.1.1',
    author='Robert Gale',
    author_email='galer@ohsu.edu',
    packages=setuptools.find_packages(),
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
