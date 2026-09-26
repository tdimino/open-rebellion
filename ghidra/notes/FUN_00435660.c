
void __thiscall FUN_00435660(void *this,void *param_1)

{
  void *pvVar1;
  undefined4 *puVar2;
  void *local_18;
  undefined4 local_14;
  undefined4 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d3eb;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f4d90(param_1,&local_14);
  *(undefined4 *)((int)this + 0xc) = local_14;
  FUN_005f4d90(param_1,&local_18);
  param_1 = (void *)0x0;
  if (local_18 != (void *)0x0) {
    do {
      local_10 = (undefined4 *)FUN_00618b70(0x20);
      local_4 = 0;
      if (local_10 == (undefined4 *)0x0) {
        puVar2 = (undefined4 *)0x0;
      }
      else {
        puVar2 = FUN_0041be00(local_10);
      }
      local_4 = 0xffffffff;
      if (puVar2 != (undefined4 *)0x0) {
        (**(code **)(puVar2[3] + 4))(pvVar1);
        FUN_00435790(this,(int)puVar2);
      }
      param_1 = (void *)((int)param_1 + 1);
    } while (param_1 < local_18);
  }
  ExceptionList = local_c;
  return;
}

