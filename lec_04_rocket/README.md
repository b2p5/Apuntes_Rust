# API de Visualización de la Mempool de Bitcoin

Esta API proporciona una interfaz simple para visualizar las transacciones en la mempool de Bitcoin( en modo prueba, sin conexión al nodo ). Utiliza Rocket en Rust para crear un servidor web que responde con la lista actual de transacciones en la mempool.

## Características

- **Listar Transacciones en la Mempool**: Endpoint para obtener las transacciones actuales en la mempool de Bitcoin.

## Requisitos Previos

Para ejecutar este proyecto, necesitas tener instalado Rust y el toolchain de Rust Nightly debido a las dependencias de Rocket. Puedes instalar Rust desde [rustup](https://rustup.rs/).

Una vez instalado Rust, configura tu proyecto para usar Rust Nightly con el siguiente comando en el directorio del proyecto:


## Configuración

Para configurar el proyecto, sigue estos pasos:

1. Clona el repositorio:

git clone <URL del repositorio>

2. Cambia al directorio del proyecto:

cd <dir del proyecto>

3. Asegúrate de estar utilizando Rust Nightly en el directorio del proyecto:

rustup override set nightly


Después de ejecutar el comando, la API estará disponible en `http://localhost:8000/`.

## Endpoints

La API tiene el siguiente endpoint disponible:

- **GET /api/mempool**: Devuelve una lista de transacciones en la mempool de Bitcoin.

## Desarrollo Futuro

Este proyecto es un ejemplo básico y puede ser extendido con características adicionales, como:

- Utilizar conexión al nodo y obtener Txs reales.
- Filtrado y búsqueda de transacciones específicas en la mempool.
- Autenticación y autorización para proteger endpoints.
- Interfaz de usuario para visualizar las transacciones.

## Contribuir

Si estás interesado en contribuir al proyecto, por favor lee `CONTRIBUTING.md` (si existe) para más detalles sobre cómo hacerlo.

## Licencia

Este proyecto está licenciado bajo la Licencia MIT - vea el archivo `LICENSE` para más detalles.



## Contacto

Si tiene alguna pregunta o sugerencia, no dude en contactarme en [Correo/Perfil de GitHub].

---
