
void __thiscall FUN_0059c1c0(void *this,undefined4 *param_1)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  void *pvVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065197b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar2 = FUN_00595c40();
  if (iVar2 == 1) {
    bVar1 = FUN_00595b50();
    if (CONCAT31(extraout_var,bVar1) == 1) {
      pvVar3 = (void *)FUN_00618b70(0xa4);
      local_4 = 0;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        pvVar3 = FUN_005c4df0(pvVar3,*param_1,this);
      }
      local_4 = 0xffffffff;
      *(void **)((int)this + 0x110) = pvVar3;
      if (pvVar3 != (void *)0x0) {
        FUN_005ffce0(pvVar3,0);
        (**(code **)(**(int **)((int)this + 0x110) + 4))(5);
      }
    }
  }
  ExceptionList = local_c;
  return;
}

