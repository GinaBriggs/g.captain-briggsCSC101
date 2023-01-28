--
-- PostgreSQL database dump
--

-- Dumped from database version 15.1
-- Dumped by pg_dump version 15.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
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
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_age integer NOT NULL,
    c_email character(50),
    c_mobile bigint NOT NULL,
    eid integer NOT NULL,
    data_id integer NOT NULL,
    c_name text
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (c_id, c_age, c_email, c_mobile, eid, data_id, c_name) FROM stdin;
110	35	m_karim@gmail.com                                 	8055089112	102	5	Musta Karim
111	43	l_jaiye@gmail.com                                 	8055185341	100	3	Lilian Jaiva
112	50	a_musa@gmail.com                                  	7055282813	107	10	Arthur Musa
113	41	p_akonjo@gmail.com                                	9052356772	100	2	Philip Akonjo
114	33	m_mapa@gmail.com                                  	8053333551	120	5	Marylene Mapa
115	50	o_agor@gmail.com                                  	7055566774	117	11	Oghenero Agor
116	33	a_bree@gmail.com                                  	8056765424	102	1	Adams Bree
117	45	o_mathias@gmail.com                               	8056763367	120	10	Okafor Mathias
118	65	s_adeleke@gmail.com                               	7056774423	117	11	Samson Adeleke
119	35	l_tamire@gmail.com                                	9052111101	107	5	Lawal Tamire
120	44	j_job@gmail.com                                   	8059693919	100	8	James Job
121	21	m_jakande@gmail.com                               	7051232144	120	2	Matthew Jakande
122	20	j_adegboye@gmail.com                              	8054921923	107	5	Jimila Adegboye
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- PostgreSQL database dump complete
--

