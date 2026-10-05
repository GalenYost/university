--
-- PostgreSQL database dump
--

\restrict X8pyOetMkp2fnhbUBoPCB1QEgUTzrJcgUHiEOQgsPrdIveO58PZXFdvcSLgEvCw

-- Dumped from database version 17.11
-- Dumped by pg_dump version 17.11

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: clients; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.clients (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    name text NOT NULL,
    activity_type text NOT NULL,
    address text NOT NULL,
    phone text NOT NULL
);


ALTER TABLE public.clients OWNER TO postgres;

--
-- Name: deals; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.deals (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    amount numeric(18,2) NOT NULL,
    commission numeric(18,2) NOT NULL,
    description text NOT NULL,
    client_id uuid NOT NULL,
    service_id uuid NOT NULL
);


ALTER TABLE public.deals OWNER TO postgres;

--
-- Name: services; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.services (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    name text NOT NULL,
    description text NOT NULL
);


ALTER TABLE public.services OWNER TO postgres;

--
-- Data for Name: clients; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.clients (id, name, activity_type, address, phone) FROM stdin;
01a0edf3-eeea-7db2-8ee2-c44e7d462bc2	client1	activity1	address1	+380996611117
01a0edf4-7f6a-7516-aed6-36fb4c3d77cb	client2	student	unknown	+380996622227
01a0ee2e-ee0f-7fde-876a-19324ebfc0ca	ТОВ клієнт 2	IT	unknown	+380994848021
01a0fc23-06b5-7720-85e7-a87f09365942	клієнт3		123 тест	123456
01a0fc23-4504-70d8-94fa-61796d12c86f	клієнт4		123456	123456
\.


--
-- Data for Name: deals; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.deals (id, amount, commission, description, client_id, service_id) FROM stdin;
01a0edf5-ae01-7c33-8070-4d7156970eff	10000.00	500.00	client1 - two	01a0edf3-eeea-7db2-8ee2-c44e7d462bc2	01a0edf5-4c91-743b-8820-276df824be1c
01a0edf5-e663-7939-b0a2-0de1459327cd	15000.00	1500.00	client2 - two	01a0edf4-7f6a-7516-aed6-36fb4c3d77cb	01a0edf5-4c91-743b-8820-276df824be1c
01a0edf6-34a7-76bf-bc62-1f26d983c008	100000.00	9000.00	client2 - one	01a0edf4-7f6a-7516-aed6-36fb4c3d77cb	01a0edf5-2d6f-72e2-ae57-fa7b37635e0a
01a0fc0a-19e9-7bc2-b5bc-fe2c7979eb2d	220000.00	10000.00	123 test	01a0edf4-7f6a-7516-aed6-36fb4c3d77cb	01a0edf5-4c91-743b-8820-276df824be1c
01a0fc0a-c069-7d02-acd6-c7213a04c606	350000.00	5000.00	test	01a0edf3-eeea-7db2-8ee2-c44e7d462bc2	01a0edf5-2d6f-72e2-ae57-fa7b37635e0a
01a0fc26-04c1-76be-a1ef-e8611ada50fe	5000.00	0.00	test	01a0fc23-4504-70d8-94fa-61796d12c86f	01a0fc20-3d6e-7ed2-9661-273a139cc0c1
\.


--
-- Data for Name: services; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.services (id, name, description) FROM stdin;
01a0edf5-2d6f-72e2-ae57-fa7b37635e0a	one	test one
01a0edf5-4c91-743b-8820-276df824be1c	two	test two
01a0fc1f-a7bb-711f-af10-52621f937133	послуга1	опис
01a0fc1f-c7d3-7047-981e-c752269b7ede	інша послуга	1234
01a0fc20-22e5-72c9-9936-fc7a926632c4	послуга2	тест
01a0fc20-3d6e-7ed2-9661-273a139cc0c1	456	123
\.


--
-- Name: clients clients_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.clients
    ADD CONSTRAINT clients_pkey PRIMARY KEY (id);


--
-- Name: deals deals_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.deals
    ADD CONSTRAINT deals_pkey PRIMARY KEY (id);


--
-- Name: services services_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.services
    ADD CONSTRAINT services_pkey PRIMARY KEY (id);


--
-- Name: deals deals_client_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.deals
    ADD CONSTRAINT deals_client_id_fkey FOREIGN KEY (client_id) REFERENCES public.clients(id) ON DELETE CASCADE;


--
-- Name: deals deals_service_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.deals
    ADD CONSTRAINT deals_service_id_fkey FOREIGN KEY (service_id) REFERENCES public.services(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

\unrestrict X8pyOetMkp2fnhbUBoPCB1QEgUTzrJcgUHiEOQgsPrdIveO58PZXFdvcSLgEvCw

