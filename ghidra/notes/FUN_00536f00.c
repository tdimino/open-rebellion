
void __thiscall FUN_00536f00(void *this,void *param_1)

{
  void *pvVar1;
  int *piVar2;
  void *local_4;
  
  pvVar1 = param_1;
  local_4 = this;
  FUN_005f4d90(param_1,&param_1);
  while (param_1 != (void *)0x0) {
    param_1 = (void *)((int)param_1 + -1);
    FUN_005f4d90(pvVar1,&local_4);
    piVar2 = (int *)FUN_004f5cd0((int)local_4);
    if (piVar2 != (int *)0x0) {
      (**(code **)(*piVar2 + 4))(pvVar1);
      FUN_00536fe0(this,piVar2);
    }
  }
  return;
}

