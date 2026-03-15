
void * __thiscall
FUN_004a2630(void *this,undefined4 param_1,int param_2,uint param_3,int param_4,undefined4 param_5,
            void *param_6)

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
  puStack_8 = &LAB_00637930;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ac120(this,param_1,param_2,param_3,0xeb,0x130,param_4,param_5,(uint *)0x0);
  local_4 = 0;
  FUN_004ece30((undefined4 *)((int)this + 0x144));
  local_4._0_1_ = 1;
  FUN_005f4950((void *)((int)this + 0x14c),0);
  local_4._0_1_ = 2;
  FUN_00604490((undefined4 *)((int)this + 0x150));
  *(uint *)((int)this + 0x50) = *(uint *)((int)this + 0x50) | 0x2000000;
  bVar5 = param_6 == (void *)0x0;
  local_4._0_1_ = 3;
  uVar1 = (undefined1)local_4;
  local_4._0_1_ = 3;
  *(undefined ***)this = &PTR_FUN_0065bda0;
  *(uint *)((int)this + 0xb8) = *(uint *)((int)this + 0xb8) | 4;
  if (bVar5) {
    local_4._0_1_ = uVar1;
    puVar2 = (uint *)FUN_004ece30(&param_5);
    local_4 = 5;
  }
  else {
    puVar2 = FUN_004025b0(param_6,&param_3);
    local_4 = CONCAT31(local_4._1_3_,4);
  }
  FUN_004f26d0((undefined4 *)((int)this + 0x144),puVar2);
  local_4 = 4;
  if (bVar5) {
    FUN_00619730();
  }
  local_4 = 3;
  if (!bVar5) {
    FUN_00619730();
  }
  *(int *)((int)this + 0x148) = param_4;
  *(undefined4 *)((int)this + 0x158) = 0;
  *(undefined4 *)((int)this + 0x15c) = 0;
  *(undefined4 *)((int)this + 0x160) = 0;
  *(undefined4 *)((int)this + 0x164) = 0;
  *(undefined4 *)((int)this + 0x168) = 0;
  *(undefined4 *)((int)this + 0x178) = 0;
  *(undefined4 *)((int)this + 0x17c) = 0;
  *(undefined4 *)((int)this + 0x194) = 0x66;
  *(undefined4 *)((int)this + 0x1a4) = 0;
  *(undefined4 *)((int)this + 0x1a8) = 0;
  *(undefined4 *)((int)this + 0x1ac) = 0;
  iVar3 = FUN_005fefd0(10);
  *(int *)((int)this + 0x1b0) = iVar3;
  if (iVar3 == 0) {
    pBVar4 = FUN_00401440();
    iVar3 = FUN_005ff020((char *)pBVar4,s_gokres_dll_006a8708,10);
    *(int *)((int)this + 0x1b0) = iVar3;
    if (pBVar4 != (LPBYTE)0x0) {
      FUN_00618b60(pBVar4);
    }
  }
  ExceptionList = local_c;
  return this;
}

