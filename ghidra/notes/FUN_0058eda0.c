
undefined4 __thiscall FUN_0058eda0(void *this,void *param_1)

{
  undefined4 *puVar1;
  void *pvVar2;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar2 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00650260;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)FUN_00402e40(param_1,(uint *)&param_1);
  local_4 = 0;
  FUN_004f26d0((void *)((int)this + 0x20),puVar1);
  local_4 = 0xffffffff;
  FUN_00619730();
  param_1 = *(void **)((int)pvVar2 + 0x30);
  pvVar2 = FUN_005f2fc0(local_18,&param_1);
  local_4 = 1;
  FUN_005f3090((void *)((int)this + 0x24),(int)pvVar2);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  ExceptionList = local_c;
  return *(undefined4 *)((int)this + 0x2c);
}

