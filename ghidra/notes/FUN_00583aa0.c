
uint __thiscall FUN_00583aa0(void *this,void *param_1)

{
  void *pvVar1;
  undefined4 *puVar2;
  int *piVar3;
  uint uVar4;
  int local_14;
  undefined4 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f1df;
  local_c = ExceptionList;
  uVar4 = 1;
  ExceptionList = &local_c;
  FUN_005f5ee0((int)this);
  FUN_005f4d90(param_1,(int)this + 0x10);
  FUN_005f4d90(param_1,&local_14);
  param_1 = (void *)0x0;
  if (0 < local_14) {
    do {
      puVar2 = (undefined4 *)FUN_00618b70(0x18);
      if (puVar2 == (undefined4 *)0x0) {
        FUN_00619730();
      }
      local_4 = 0;
      local_10 = puVar2;
      if (puVar2 == (undefined4 *)0x0) {
        piVar3 = (int *)0x0;
      }
      else {
        piVar3 = FUN_00583c70(puVar2);
      }
      local_4 = 0xffffffff;
      uVar4 = 0;
      if ((piVar3 != (int *)0x0) && (uVar4 = piVar3[3], uVar4 != 0)) {
        (**(code **)(*piVar3 + 4))(pvVar1);
        FUN_00583c50(this,(int)piVar3);
      }
      param_1 = (void *)((int)param_1 + 1);
    } while ((uVar4 & (int)param_1 < local_14) != 0);
  }
  ExceptionList = local_c;
  return uVar4;
}

