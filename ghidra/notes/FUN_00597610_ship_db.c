
undefined4 * __fastcall FUN_00597610(undefined4 *param_1)

{
  undefined4 *this;
  int iVar1;
  undefined4 *puVar2;
  undefined4 local_7e4 [3];
  undefined4 *local_7d8;
  undefined1 local_7d4 [456];
  undefined1 local_60c [1536];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006516e7;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_7d8 = param_1;
  FUN_00618c80(param_1 + 4,0xc,0xc,FUN_005f2f50);
  local_4 = 0;
  FUN_00618c80(param_1 + 0x28,0xc,0x26,FUN_005f2f50);
  this = param_1 + 0x1a2;
  local_4._0_1_ = 1;
  FUN_005f4ed0(this);
  *this = &PTR_FUN_0066baf0;
  local_4._0_1_ = 2;
  param_1[0x1a4] = 0;
  FUN_005fbb20(param_1 + 0x1a6);
  *param_1 = &PTR_FUN_0066bae8;
  param_1[2] = 0;
  param_1[0x1a5] = 1;
  iVar1 = param_1[0x1a4];
  local_4._0_1_ = 3;
  param_1[0x1a4] = iVar1 + 1;
  if (iVar1 + 1 < 0) {
    param_1[0x1a4] = 0;
  }
  param_1[0x1ac] = 1;
  param_1[0x19b] = 0;
  param_1[0x19d] = 0;
  param_1[0x19c] = 0;
  param_1[0x19e] = 0;
  param_1[0x1a1] = 0;
  param_1[0x19f] = 0;
  param_1[0x1a0] = 0;
  param_1[0x19a] = 0;
  FUN_005f35b0(local_7e4,s_CALAMARI_006ab6d8);
  local_4._0_1_ = 4;
  FUN_005f3090(param_1 + 0x28,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_BULK_CRUISER_006ab6c8);
  local_4._0_1_ = 5;
  FUN_005f3090(param_1 + 0x2b,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_ASSAULT_FRIGATE_006ab6b8);
  local_4._0_1_ = 6;
  FUN_005f3090(param_1 + 0x2e,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_NEBULON_B_006ab6ac);
  local_4._0_1_ = 7;
  FUN_005f3090(param_1 + 0x31,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_ESCORT_A_006ab6a0);
  local_4._0_1_ = 8;
  FUN_005f3090(param_1 + 0x34,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_CORVETTE_006ab694);
  local_4._0_1_ = 9;
  FUN_005f3090(param_1 + 0x37,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_MEDIUM_TRANSPORT_006ab680);
  local_4._0_1_ = 10;
  FUN_005f3090(param_1 + 0x3a,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_BULK_TRANSPORT_006ab670);
  local_4._0_1_ = 0xb;
  FUN_005f3090(param_1 + 0x3d,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_GUNBOAT_006ab668);
  local_4._0_1_ = 0xc;
  FUN_005f3090(param_1 + 0x40,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_DREADNAUGHT_A_006ab658);
  local_4._0_1_ = 0xd;
  FUN_005f3090(param_1 + 0x43,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_CC7700_006ab650);
  local_4._0_1_ = 0xe;
  FUN_005f3090(param_1 + 0x46,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_CC9600_006ab648);
  local_4._0_1_ = 0xf;
  FUN_005f3090(param_1 + 0x49,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_BULWARK_006ab640);
  local_4._0_1_ = 0x10;
  FUN_005f3090(param_1 + 0x4c,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_HYDRA_006ab638);
  local_4._0_1_ = 0x11;
  FUN_005f3090(param_1 + 0x4f,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_DAUNTLESS_006ab62c);
  local_4._0_1_ = 0x12;
  FUN_005f3090(param_1 + 0x52,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_STRIKE_CRUISER_006ab61c);
  local_4._0_1_ = 0x13;
  FUN_005f3090(param_1 + 0x55,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_LANCER_006ab614);
  local_4._0_1_ = 0x14;
  FUN_005f3090(param_1 + 0x58,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_INTERDICTOR_006ab608);
  local_4._0_1_ = 0x15;
  FUN_005f3090(param_1 + 0x5b,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_CARRAK_006ab600);
  local_4._0_1_ = 0x16;
  FUN_005f3090(param_1 + 0x5e,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_VICTORY_006ab5f8);
  local_4._0_1_ = 0x17;
  FUN_005f3090(param_1 + 0x61,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_STAR_DESTROYER_006ab5e8);
  local_4._0_1_ = 0x18;
  FUN_005f3090(param_1 + 100,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_SUPER_DESTROYER_006ab5d8);
  local_4._0_1_ = 0x19;
  FUN_005f3090(param_1 + 0x67,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_ASSAULT_TRANSPORT_006ab5c4);
  local_4._0_1_ = 0x1a;
  FUN_005f3090(param_1 + 0x6a,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_GALLEON_006ab5bc);
  local_4._0_1_ = 0x1b;
  FUN_005f3090(param_1 + 0x6d,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_VICTORY_II_006ab5b0);
  local_4._0_1_ = 0x1c;
  FUN_005f3090(param_1 + 0x70,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_DESTROYER_II_006ab5a0);
  local_4._0_1_ = 0x1d;
  FUN_005f3090(param_1 + 0x73,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_STAR_GALLEON_006ab590);
  local_4._0_1_ = 0x1e;
  FUN_005f3090(param_1 + 0x76,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_ESCORT_E_006ab584);
  local_4._0_1_ = 0x1f;
  FUN_005f3090(param_1 + 0x79,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_DREADNAUGHT_E_006ab574);
  local_4._0_1_ = 0x20;
  FUN_005f3090(param_1 + 0x7c,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_XWING_006ab56c);
  local_4._0_1_ = 0x21;
  FUN_005f3090(param_1 + 0x7f,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_YWING_006ab564);
  local_4._0_1_ = 0x22;
  FUN_005f3090(param_1 + 0x82,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_AWING_006ab55c);
  local_4._0_1_ = 0x23;
  FUN_005f3090(param_1 + 0x85,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_BWING_006ab554);
  local_4._0_1_ = 0x24;
  FUN_005f3090(param_1 + 0x88,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_TIE_FIGHTER_006ab548);
  local_4._0_1_ = 0x25;
  FUN_005f3090(param_1 + 0x8b,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_TIE_INTERCEPTOR_006ab538);
  local_4._0_1_ = 0x26;
  FUN_005f3090(param_1 + 0x8e,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_TIE_BOMBER_006ab52c);
  local_4._0_1_ = 0x27;
  FUN_005f3090(param_1 + 0x91,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_ASSAULT_GUNSHIP_006ab51c);
  local_4._0_1_ = 0x28;
  FUN_005f3090(param_1 + 0x94,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_DEATH_STAR_006ab510);
  local_4._0_1_ = 0x29;
  FUN_005f3090(param_1 + 0x97,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_ssNONE_006ab508);
  local_4._0_1_ = 0x2a;
  FUN_005f3090(param_1 + 4,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_ssDOCKED_006ab4fc);
  local_4._0_1_ = 0x2b;
  FUN_005f3090(param_1 + 7,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_ssACTIVE_006ab4f0);
  local_4._0_1_ = 0x2c;
  FUN_005f3090(param_1 + 10,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_ssLAUNCHING_006ab4e4);
  local_4._0_1_ = 0x2d;
  FUN_005f3090(param_1 + 0xd,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_ssRECOVERING_006ab4d4);
  local_4._0_1_ = 0x2e;
  FUN_005f3090(param_1 + 0x10,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_ssRECOVERED_006ab4c8);
  local_4._0_1_ = 0x2f;
  FUN_005f3090(param_1 + 0x13,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_ssHYPERINGOUT_006ab4b8);
  local_4._0_1_ = 0x30;
  FUN_005f3090(param_1 + 0x16,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_ssHYPEROUT_006ab4ac);
  local_4._0_1_ = 0x31;
  FUN_005f3090(param_1 + 0x19,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_ssWITHDRAWING_006ab49c);
  local_4._0_1_ = 0x32;
  FUN_005f3090(param_1 + 0x1c,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_ssWITHDRAWN_006ab490);
  local_4._0_1_ = 0x33;
  FUN_005f3090(param_1 + 0x1f,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_ssDEATHSTAR_006ab484);
  local_4._0_1_ = 0x34;
  FUN_005f3090(param_1 + 0x22,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  FUN_005f35b0(local_7e4,s_ssDEAD_006ab47c);
  local_4._0_1_ = 0x35;
  FUN_005f3090(param_1 + 0x25,(int)local_7e4);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_7e4);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x36;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x840;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8ba;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x37;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x841;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8c1;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x38;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x842;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8b6;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x39;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x843;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8cf;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x3a;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x850;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8b5;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x3b;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x880;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8a6;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x3c;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x881;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8d4;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x3d;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa40;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x899;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x3e;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa41;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x89c;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x3f;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa42;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x89d;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x40;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa43;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x89e;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x41;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa44;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8a2;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x42;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa45;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8b7;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x43;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa46;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8ca;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x44;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa47;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x89a;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x45;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa48;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8a4;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x46;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa49;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8a9;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x47;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa4a;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8c9;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x48;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa4b;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8a1;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x49;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa4c;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8ac;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x4a;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa4d;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8b1;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x4b;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa4e;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8bb;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x4c;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa4f;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8bd;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x4d;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa50;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8c5;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x4e;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa51;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8ce;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x4f;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa52;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8d0;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x50;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa53;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8d7;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x51;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa54;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8d8;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x52;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa55;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8af;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x53;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa56;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x89b;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x54;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa57;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8c8;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x55;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa58;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8c3;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x56;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa59;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8b8;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x57;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa80;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8ae;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x58;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa81;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8c4;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x59;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa82;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8c7;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x5a;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa83;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8d5;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x5b;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa84;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8da;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x5c;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa85;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x89f;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x5d;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa86;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8a3;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x5e;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa87;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8a7;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x5f;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa88;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8a8;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x60;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa89;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8ab;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x61;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa8a;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8aa;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x62;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa8b;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8b2;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 99;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa8c;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8bc;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 100;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa8d;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8bf;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x65;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa8e;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8c2;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x66;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa8f;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8c6;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x67;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa90;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8cd;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x68;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa91;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8d2;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x69;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa92;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8d9;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x6a;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa93;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8a0;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x6b;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa94;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8d1;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x6c;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa95;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8b4;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x6d;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa96;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8cb;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x6e;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa97;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8c0;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x6f;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa98;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8ad;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x70;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa99;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8b9;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x71;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa9a;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8b3;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x72;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0xa9b;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8d6;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x73;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x540;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x904;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x74;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x541;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x905;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x75;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x542;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x90e;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x76;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x543;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8fe;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x77;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x580;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x909;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x78;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x581;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x901;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x79;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x582;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8ff;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x7a;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x583;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x908;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x7b;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x441;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x8fd;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x7c;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x440;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x906;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x7d;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x442;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x90a;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x7e;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x443;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x90c;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x7f;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x444;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x907;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x80;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x480;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x90d;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x81;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x481;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x903;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x82;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x482;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x902;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x83;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x483;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x90b;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_00618b70(0x1c);
  local_4._0_1_ = 0x84;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5c10(puVar2);
    *(undefined2 *)(puVar2 + 6) = 0x484;
    *(undefined2 *)((int)puVar2 + 0x1a) = 0x900;
    *puVar2 = &PTR_FUN_0066bae4;
  }
  local_4._0_1_ = 3;
  FUN_005f4f10(this,(int)puVar2);
  FUN_00618c80(local_60c,0xc,0x80,FUN_005f2f50);
  local_4._0_1_ = 0x85;
  FUN_00618c80(local_7d4,0xc,0x26,FUN_005f2f50);
  param_1[0x9a] = 0x740;
  param_1[0x9b] = 0x741;
  param_1[0x9c] = 0x742;
  param_1[0x9d] = 0x743;
  param_1[0x9e] = 0x744;
  param_1[0x9f] = 0x745;
  param_1[0xa0] = 0x746;
  param_1[0xa1] = 0x747;
  param_1[0xa2] = 0x748;
  param_1[0xa3] = 0x749;
  param_1[0xa4] = 0x74a;
  param_1[0xa5] = 0x74d;
  param_1[0xa6] = 0x74b;
  param_1[0xa7] = 0x74c;
  param_1[0xa8] = 0x74e;
  param_1[0xa9] = 0x780;
  param_1[0xaa] = 0x781;
  param_1[0xab] = 0x782;
  param_1[0xac] = 0x783;
  param_1[0xad] = 0x784;
  param_1[0xae] = 0x785;
  param_1[0xaf] = 0x786;
  param_1[0xb0] = 0x787;
  param_1[0xbf] = 0x788;
  param_1[0xb1] = 0x789;
  param_1[0xb2] = 0x78a;
  param_1[0xb3] = 0x78b;
  param_1[0xb4] = 0x78c;
  param_1[0xb5] = 0x78d;
  param_1[0xb6] = 0x78e;
  param_1[0xb7] = 0x642;
  param_1[0xb8] = 0x643;
  param_1[0xb9] = 0x640;
  param_1[0xba] = 0x641;
  param_1[0xbb] = 0x680;
  param_1[0xbc] = 0x681;
  param_1[0xbd] = 0x682;
  param_1[0xbe] = 0x683;
  FUN_00618d20(local_7d4,0xc,0x26,FUN_005f2ff0);
  local_4 = CONCAT31(local_4._1_3_,3);
  FUN_00618d20(local_60c,0xc,0x80,FUN_005f2ff0);
  ExceptionList = local_c;
  return param_1;
}

