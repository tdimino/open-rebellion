
void __fastcall FUN_00485850(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0063483e;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0065abf0;
  local_4 = 2;
  FUN_00417b40((int)(param_1 + 0x21));
  local_4._0_1_ = 1;
  FUN_004be820(param_1 + 0x19);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004bee90(param_1 + 0x15);
  local_4 = 0xffffffff;
  FUN_00488580(param_1);
  ExceptionList = pvStack_c;
  return;
}

