
bool __cdecl FUN_0053f470(void *param_1)

{
  uint *puVar1;
  bool bVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00646938;
  local_c = ExceptionList;
  bVar2 = DAT_006b90e8 != (void *)0x0;
  if ((bVar2) && (*(int *)((int)DAT_006b90e8 + 4) != 0)) {
    ExceptionList = &local_c;
    puVar1 = FUN_004025b0(param_1,(uint *)&param_1);
    local_4 = 0;
    FUN_004f5860(DAT_006b90e8,puVar1);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return bVar2;
}

