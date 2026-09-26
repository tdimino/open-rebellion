
uint __thiscall FUN_0052a380(void *this,int param_1,int *param_2,void *param_3)

{
  void *pvVar1;
  uint uVar2;
  void *this_00;
  int *piVar3;
  void *pvVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_3;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00644948;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  uVar2 = FUN_005298d0(this,param_2,param_3);
  if (uVar2 != 0) {
    this_00 = (void *)thunk_FUN_005f5060(param_1);
    do {
      if (this_00 == (void *)0x0) {
        ExceptionList = local_c;
        return uVar2;
      }
      piVar3 = (int *)FUN_00504dc0((uint *)((int)this_00 + 0x1c));
      uVar2 = 0;
      if (piVar3 != (int *)0x0) {
        pvVar4 = FUN_00403040(this_00,&param_3);
        local_4 = 0;
        uVar2 = (**(code **)(*piVar3 + 0xa8))(pvVar4,pvVar1);
        local_4 = 0xffffffff;
        FUN_00619730();
      }
      this_00 = *(void **)((int)this_00 + 0x10);
    } while (uVar2 != 0);
  }
  ExceptionList = local_c;
  return 0;
}

