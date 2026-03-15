
void * __thiscall
FUN_00406950(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4,
            int param_5,undefined4 param_6,undefined4 param_7)

{
  uint uVar1;
  uint uVar2;
  undefined4 *puVar3;
  undefined4 uVar4;
  LPBYTE pBVar5;
  int iVar6;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006298c8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005ff1c0(this,param_1,param_2,param_3,param_4,param_5,param_6,0,0);
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_00658600;
  *(undefined4 *)((int)this + 0xe0) = 0;
  *(undefined4 *)((int)this + 0xdc) = 0;
  *(undefined4 *)((int)this + 0xe4) = 0;
  *(undefined4 *)((int)this + 0xe8) = 0;
  *(undefined4 *)((int)this + 0xec) = 0;
  *(undefined4 *)((int)this + 0xf0) = 0;
  *(undefined4 *)((int)this + 0xf4) = 0;
  *(undefined4 *)((int)this + 0xf8) = 0;
  *(undefined4 *)((int)this + 0xfc) = 0;
  *(undefined4 *)((int)this + 0x100) = 0;
  *(undefined4 *)((int)this + 0x104) = 0;
  *(undefined4 *)((int)this + 0x108) = 0;
  *(undefined4 *)((int)this + 0x10c) = 0;
  *(undefined4 *)((int)this + 0x110) = 0;
  *(undefined4 *)((int)this + 0x114) = 0;
  *(undefined4 *)((int)this + 0x118) = 0;
  *(undefined4 *)((int)this + 0x9c) = param_7;
  *(undefined4 *)((int)this + 0xa8) = 0;
  uVar1 = FUN_00401a00();
  uVar2 = FUN_004019e0();
  puVar3 = (undefined4 *)((int)this + 0xac);
  *(uint *)((int)this + 0xa0) = uVar1 | uVar2;
  iVar6 = 6;
  do {
    puVar3[6] = 0;
    *puVar3 = 0;
    puVar3 = puVar3 + 1;
    iVar6 = iVar6 + -1;
  } while (iVar6 != 0);
  uVar4 = FUN_00401c10();
  *(undefined4 *)((int)this + 0xa8) = uVar4;
  pBVar5 = FUN_00401440();
  iVar6 = FUN_006037f0(1);
  if (iVar6 == 0) {
    FUN_005ff020((char *)pBVar5,s_textcomm_dll_006a7edc,1);
  }
  FUN_005ff020((char *)pBVar5,s_common_dll_006a7c98,8);
  if (pBVar5 != (LPBYTE)0x0) {
    FUN_00618b60(pBVar5);
  }
  ExceptionList = local_c;
  return this;
}

