
void * __thiscall
FUN_0046cbc0(void *this,undefined4 param_1,int param_2,uint param_3,int param_4,int param_5,
            int param_6,undefined4 param_7)

{
  undefined4 *this_00;
  int iVar1;
  LPBYTE pBVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00632011;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ac120(this,param_1,param_2,param_3,param_4,param_5,param_6,param_7,(uint *)0x0);
  local_4 = 0;
  FUN_004ece30((undefined4 *)((int)this + 0x17c));
  local_4._0_1_ = 1;
  *(undefined ***)this = &PTR_FUN_0065a270;
  *(undefined4 *)((int)this + 0x94) = 2;
  *(undefined4 *)((int)this + 0x98) = 7;
  *(uint *)((int)this + 0xb8) = *(uint *)((int)this + 0xb8) | 1;
  *(undefined4 *)((int)this + 0x144) = 0;
  *(undefined4 *)((int)this + 0x148) = 0;
  *(undefined4 *)((int)this + 0x14c) = 0;
  *(undefined4 *)((int)this + 0x154) = 0;
  *(int *)((int)this + 0x150) = param_6;
  *(undefined4 *)((int)this + 0x158) = 0;
  this_00 = (undefined4 *)FUN_00618b70(0x10);
  local_4._0_1_ = 2;
  if (this_00 == (undefined4 *)0x0) {
    this_00 = (undefined4 *)0x0;
  }
  else {
    FUN_0060a790(this_00,2);
    *this_00 = &PTR_FUN_0065a120;
  }
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined4 **)((int)this + 0x178) = this_00;
  iVar1 = FUN_005fefd0(10);
  *(int *)((int)this + 0x15c) = iVar1;
  if (iVar1 == 0) {
    pBVar2 = FUN_00401440();
    iVar1 = FUN_005ff020((char *)pBVar2,s_gokres_dll_006a8708,10);
    *(int *)((int)this + 0x15c) = iVar1;
    if (pBVar2 != (LPBYTE)0x0) {
      FUN_00618b60(pBVar2);
    }
  }
  *(undefined4 *)((int)this + 0x160) = 0;
  ExceptionList = local_c;
  return this;
}

