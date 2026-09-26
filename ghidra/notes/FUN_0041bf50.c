
void __thiscall FUN_0041bf50(void *this,void *param_1)

{
  void *pvVar1;
  int *piVar2;
  void *local_14;
  undefined4 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b69b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5bd0(this,param_1);
  FUN_005f4d90(param_1,(int)this + 0x10);
  FUN_005f4d90(param_1,&local_14);
  param_1 = (void *)0x0;
  if (local_14 != (void *)0x0) {
    do {
      local_10 = (undefined4 *)FUN_00618b70(0x18);
      local_4 = 0;
      if (local_10 == (undefined4 *)0x0) {
        piVar2 = (int *)0x0;
      }
      else {
        piVar2 = FUN_0041c2e0(local_10);
      }
      local_4 = 0xffffffff;
      if (piVar2 != (int *)0x0) {
        (**(code **)(*piVar2 + 4))(pvVar1);
        FUN_0041c070((void *)((int)this + -0xc),piVar2);
      }
      param_1 = (void *)((int)param_1 + 1);
    } while (param_1 < local_14);
  }
  ExceptionList = local_c;
  return;
}

