
void __thiscall FUN_0049c6f0(void *this,void *param_1)

{
  void *pvVar1;
  undefined4 *puVar2;
  int *piVar3;
  void *pvVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00636feb;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f4d90(param_1,&param_1);
  pvVar4 = (void *)0x0;
  if (param_1 != (void *)0x0) {
    do {
      puVar2 = (undefined4 *)FUN_00618b70(0x2c);
      piVar3 = (int *)0x0;
      local_4 = 0;
      if (puVar2 != (undefined4 *)0x0) {
        piVar3 = FUN_004c5230(puVar2);
      }
      local_4 = 0xffffffff;
      (**(code **)(*piVar3 + 4))(pvVar1);
      FUN_005f5440(this,piVar3);
      pvVar4 = (void *)((int)pvVar4 + 1);
    } while (pvVar4 < param_1);
  }
  ExceptionList = local_c;
  return;
}

