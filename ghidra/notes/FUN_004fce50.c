
undefined4 __cdecl FUN_004fce50(void *param_1)

{
  void *pvVar1;
  undefined4 local_20 [5];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006402b8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_0051ce00();
  if (pvVar1 != (void *)0x0) {
    FUN_0051d780(pvVar1,param_1);
    ExceptionList = local_c;
    return 1;
  }
  pvVar1 = (void *)FUN_0051c740(local_20);
  local_4 = 0;
  FUN_0051c790(param_1,pvVar1);
  local_4 = 0xffffffff;
  FUN_0051c780(local_20);
  ExceptionList = local_c;
  return 1;
}

