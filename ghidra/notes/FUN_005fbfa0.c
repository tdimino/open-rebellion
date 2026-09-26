
void __fastcall FUN_005fbfa0(undefined4 *param_1)

{
  byte bVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00655f0b;
  local_c = ExceptionList;
  local_4 = 0;
  bVar1 = *(byte *)(param_1 + 3) & 0xf7;
  if (bVar1 == 2) {
    ExceptionList = &local_c;
    if ((HGLOBAL)param_1[5] != (HGLOBAL)0x0) {
      ExceptionList = &local_c;
      FreeResource((HGLOBAL)param_1[5]);
    }
  }
  else {
    ExceptionList = &local_c;
    if (((bVar1 != 4) && (ExceptionList = &local_c, bVar1 == 0x10)) &&
       (ExceptionList = &local_c, (HGDIOBJ)param_1[7] != (HGDIOBJ)0x0)) {
      ExceptionList = &local_c;
      DeleteObject((HGDIOBJ)param_1[7]);
    }
    if ((undefined *)*param_1 != (undefined *)0x0) {
      FUN_00618b60((undefined *)*param_1);
    }
  }
  if ((HGDIOBJ)param_1[8] != (HGDIOBJ)0x0) {
    DeleteObject((HGDIOBJ)param_1[8]);
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

