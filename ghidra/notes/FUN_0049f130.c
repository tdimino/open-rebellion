
void * __thiscall
FUN_0049f130(void *this,undefined4 param_1,int param_2,uint param_3,uint param_4,undefined4 param_5,
            void *param_6)

{
  uint uVar1;
  undefined1 uVar2;
  uint *puVar3;
  int iVar4;
  LPBYTE pBVar5;
  bool bVar6;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  uVar1 = param_4;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006373a0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ac120(this,param_1,param_2,param_3,0xeb,0x130,param_4,param_5,(uint *)0x0);
  local_4 = 0;
  FUN_004ece30((undefined4 *)((int)this + 0x144));
  local_4._0_1_ = 1;
  FUN_00601880((void *)((int)this + 0x17c),(char *)0x0,0,0,0,0,0,0);
  local_4._0_1_ = 2;
  FUN_004ece30((undefined4 *)((int)this + 0x1c8));
  local_4._0_1_ = 3;
  uVar2 = (undefined1)local_4;
  local_4._0_1_ = 3;
  *(undefined ***)this = &PTR_FUN_0065bd10;
  *(uint *)((int)this + 0xb8) = *(uint *)((int)this + 0xb8) | 4;
  bVar6 = param_6 == (void *)0x0;
  *(undefined4 *)((int)this + 0x94) = 2;
  *(undefined4 *)((int)this + 0x98) = 7;
  *(uint *)((int)this + 0x14c) = uVar1;
  if (bVar6) {
    local_4._0_1_ = uVar2;
    puVar3 = (uint *)FUN_004ece30(&param_5);
    local_4 = 5;
  }
  else {
    puVar3 = FUN_004025b0(param_6,&param_4);
    local_4 = CONCAT31(local_4._1_3_,4);
  }
  FUN_004f26d0((undefined4 *)((int)this + 0x144),puVar3);
  local_4 = 4;
  if (bVar6) {
    FUN_00619730();
  }
  local_4 = 3;
  if (!bVar6) {
    FUN_00619730();
  }
  *(undefined4 *)((int)this + 0x150) = 0;
  *(undefined4 *)((int)this + 0x154) = 0;
  *(undefined4 *)((int)this + 0x158) = 0;
  *(undefined4 *)((int)this + 0x164) = 0;
  *(undefined4 *)((int)this + 0x1b4) = 0;
  *(undefined4 *)((int)this + 0x1b8) = 0;
  *(undefined4 *)((int)this + 0x1c0) = 0;
  *(undefined4 *)((int)this + 0x1bc) = 0;
  *(undefined4 *)((int)this + 0x170) = 0;
  *(undefined4 *)((int)this + 0x160) = 0;
  *(undefined4 *)((int)this + 0x174) = 0;
  *(undefined4 *)((int)this + 0x178) = 0;
  *(undefined4 *)((int)this + 0x168) = 0;
  *(undefined4 *)((int)this + 0x16c) = 0;
  *(undefined4 *)((int)this + 0x1c4) = 0;
  iVar4 = FUN_005fefd0(10);
  *(int *)((int)this + 0x1b0) = iVar4;
  if (iVar4 == 0) {
    pBVar5 = FUN_00401440();
    iVar4 = FUN_005ff020((char *)pBVar5,s_gokres_dll_006a8708,10);
    *(int *)((int)this + 0x1b0) = iVar4;
    if (pBVar5 != (LPBYTE)0x0) {
      FUN_00618b60(pBVar5);
    }
  }
  ExceptionList = local_c;
  return this;
}

