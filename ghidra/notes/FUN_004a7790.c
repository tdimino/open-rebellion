
void * __thiscall
FUN_004a7790(void *this,undefined4 param_1,int param_2,uint param_3,int param_4,int param_5,
            int param_6,uint param_7,void *param_8)

{
  uint *puVar1;
  int iVar2;
  LPBYTE pBVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637f4e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ac120(this,param_1,param_2,param_3,param_4,param_5,param_6,param_7,(uint *)0x0);
  local_4 = 0;
  FUN_004ece30((undefined4 *)((int)this + 0x144));
  local_4._0_1_ = 1;
  *(undefined ***)this = &PTR_FUN_0065be30;
  *(uint *)((int)this + 0xb8) = *(uint *)((int)this + 0xb8) | 4;
  *(undefined4 *)((int)this + 0x94) = 2;
  *(undefined4 *)((int)this + 0x98) = 7;
  *(int *)((int)this + 0x14c) = param_6;
  if (param_8 != (void *)0x0) {
    puVar1 = FUN_004025b0(param_8,&param_7);
    local_4._0_1_ = 2;
    FUN_004f26d0((undefined4 *)((int)this + 0x144),puVar1);
    local_4._0_1_ = 1;
    FUN_00619730();
  }
  *(undefined4 *)((int)this + 0x158) = 0;
  *(undefined4 *)((int)this + 0x17c) = 0;
  *(undefined4 *)((int)this + 0x180) = 0;
  *(undefined4 *)((int)this + 0x16c) = 0;
  *(undefined4 *)((int)this + 0x170) = 0;
  *(undefined4 *)((int)this + 0x174) = 0;
  *(undefined4 *)((int)this + 0x178) = 0;
  iVar2 = FUN_005fefd0(10);
  *(int *)((int)this + 0x15c) = iVar2;
  if (iVar2 == 0) {
    pBVar3 = FUN_00401440();
    iVar2 = FUN_005ff020((char *)pBVar3,s_gokres_dll_006a8708,10);
    *(int *)((int)this + 0x15c) = iVar2;
    if (pBVar3 != (LPBYTE)0x0) {
      FUN_00618b60(pBVar3);
    }
  }
  ExceptionList = local_c;
  return this;
}

