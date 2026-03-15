
void * __thiscall
FUN_00452fc0(void *this,undefined4 param_1,int param_2,uint param_3,int param_4,uint param_5,
            int param_6,undefined4 param_7,void *param_8)

{
  undefined1 uVar1;
  uint *puVar2;
  int iVar3;
  LPBYTE pBVar4;
  bool bVar5;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062fdbc;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ac120(this,param_1,param_2,param_3,param_4,param_5,param_6,param_7,(uint *)0x0);
  local_4 = 0;
  FUN_004ece30((undefined4 *)((int)this + 0x144));
  local_4._0_1_ = 1;
  FUN_00618c80((int)this + 0x1a0,0x38,3,FUN_00453200);
  local_4._0_1_ = 2;
  uVar1 = (undefined1)local_4;
  local_4._0_1_ = 2;
  bVar5 = param_8 == (void *)0x0;
  *(undefined ***)this = &PTR_FUN_00659e68;
  if (bVar5) {
    local_4._0_1_ = uVar1;
    puVar2 = (uint *)FUN_004ece30(&param_7);
    local_4 = 4;
  }
  else {
    puVar2 = FUN_004025b0(param_8,&param_5);
    local_4 = CONCAT31(local_4._1_3_,3);
  }
  FUN_004f26d0((undefined4 *)((int)this + 0x144),puVar2);
  local_4 = 3;
  if (bVar5) {
    FUN_00619730();
  }
  local_4 = 2;
  if (!bVar5) {
    FUN_00619730();
  }
  *(int *)((int)this + 0x14c) = param_6;
  *(uint *)((int)this + 0xb8) = *(uint *)((int)this + 0xb8) | 4;
  *(undefined4 *)((int)this + 0x150) = 0;
  *(undefined4 *)((int)this + 0x154) = 0;
  *(undefined4 *)((int)this + 0x158) = 0;
  *(undefined4 *)((int)this + 0x160) = 0x67;
  *(undefined4 *)((int)this + 0x170) = 0;
  *(undefined4 *)((int)this + 0x188) = 0;
  *(undefined4 *)((int)this + 0x194) = 0;
  *(undefined4 *)((int)this + 0x198) = 0;
  *(undefined4 *)((int)this + 0x19c) = 0;
  *(undefined4 *)((int)this + 0x180) = 0;
  *(undefined4 *)((int)this + 0x184) = 0;
  *(undefined4 *)((int)this + 0x248) = 0;
  *(undefined4 *)((int)this + 0x15c) = 0;
  *(undefined4 *)((int)this + 0x16c) = 0;
  *(undefined4 *)((int)this + 0x18c) = 0;
  *(undefined4 *)((int)this + 400) = 0;
  *(undefined4 *)((int)this + 0x164) = 1;
  *(undefined4 *)((int)this + 0x260) = 0;
  *(undefined4 *)((int)this + 0x268) = 0;
  *(undefined4 *)((int)this + 0x264) = 0;
  *(undefined4 *)((int)this + 0x254) = 0;
  *(undefined4 *)((int)this + 600) = 0;
  *(undefined4 *)((int)this + 0x25c) = 0;
  *(undefined4 *)((int)this + 0x17c) = 0;
  FUN_00458480((int)this);
  iVar3 = FUN_005fefd0(10);
  *(int *)((int)this + 0x168) = iVar3;
  if (iVar3 == 0) {
    pBVar4 = FUN_00401440();
    iVar3 = FUN_005ff020((char *)pBVar4,s_gokres_dll_006a8708,10);
    *(int *)((int)this + 0x168) = iVar3;
    if (pBVar4 != (LPBYTE)0x0) {
      FUN_00618b60(pBVar4);
    }
  }
  ExceptionList = local_c;
  return this;
}

