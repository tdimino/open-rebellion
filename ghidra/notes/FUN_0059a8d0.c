
void * __thiscall
FUN_0059a8d0(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4,
            int param_5,undefined4 param_6,undefined4 param_7)

{
  LPBYTE pBVar1;
  int iVar2;
  undefined4 *puVar3;
  char *pcVar4;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00651898;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005ff1c0(this,param_1,param_2,param_3,param_4,param_5,param_6,0,0);
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_0066bb00;
  pBVar1 = FUN_00401440();
  FUN_005ff020((char *)pBVar1,s_texttact_dll_006ab714,3);
  FUN_005ff020((char *)pBVar1,s_tactical_dll_006ab704,6);
  iVar2 = FUN_00595c40();
  if (iVar2 == 0) {
    pcVar4 = s_VoiceFxA_dll_006ab6f4;
  }
  else {
    pcVar4 = s_VoiceFxE_dll_006ab6e4;
  }
  FUN_005ff020((char *)pBVar1,pcVar4,0xe);
  if (pBVar1 != (LPBYTE)0x0) {
    FUN_00618b60(pBVar1);
  }
  DAT_006bc45c = this;
  *(undefined **)((int)this + 0xd8) = &DAT_006bc3a0;
  *(undefined **)((int)this + 0xdc) = &DAT_006bc39c;
  *(undefined **)((int)this + 0xe0) = &DAT_006bc3c8;
  *(undefined **)((int)this + 0xe4) = &DAT_006bc3c0;
  *(undefined **)((int)this + 0xe8) = &DAT_006bc3d0;
  *(undefined **)((int)this + 0xec) = &DAT_006bc3cc;
  *(undefined **)((int)this + 0xf0) = &DAT_006bc3b4;
  *(undefined **)((int)this + 0xf4) = &DAT_006bc3ac;
  *(undefined **)((int)this + 0xf8) = &DAT_006bc434;
  *(undefined **)((int)this + 0xfc) = &DAT_006bc2c4;
  *(undefined **)((int)this + 0x100) = &DAT_006bc2f4;
  *(undefined **)((int)this + 0x104) = &DAT_006bc328;
  puVar3 = (undefined4 *)((int)this + 0xb8);
  for (iVar2 = 8; iVar2 != 0; iVar2 = iVar2 + -1) {
    *puVar3 = 0;
    puVar3 = puVar3 + 1;
  }
  *(undefined4 *)((int)this + 0xa8) = 0;
  *(undefined4 *)((int)this + 0xac) = 0;
  *(undefined4 *)((int)this + 0xb0) = 0;
  *(undefined4 *)((int)this + 0xb4) = 0;
  *(undefined4 *)((int)this + 0x9c) = param_7;
  *(undefined4 *)((int)this + 0x10c) = 0;
  *(undefined4 *)((int)this + 0xa0) = 0;
  *(undefined4 *)((int)this + 0xa4) = 0;
  *(undefined4 *)((int)this + 0x110) = 0;
  *(undefined4 *)((int)this + 0x108) = 0;
  ExceptionList = pvStack_c;
  return this;
}

