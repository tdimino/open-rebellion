
void __fastcall FUN_0060e250(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00656c88;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0066e258;
  local_4 = 0;
  if ((HGDIOBJ)param_1[0x2b] != (HGDIOBJ)0x0) {
    DeleteObject((HGDIOBJ)param_1[0x2b]);
  }
  if ((HGDIOBJ)param_1[0x2c] != (HGDIOBJ)0x0) {
    DeleteObject((HGDIOBJ)param_1[0x2c]);
  }
  if ((HGDIOBJ)param_1[0x2d] != (HGDIOBJ)0x0) {
    DeleteObject((HGDIOBJ)param_1[0x2d]);
  }
  local_4 = 0xffffffff;
  FUN_005ffb60(param_1);
  ExceptionList = local_c;
  return;
}

