
void __fastcall FUN_00616f10(int *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00657298;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = (int)&PTR_FUN_0066e59c;
  local_4 = 0;
  if (param_1[0xc] < 0) {
    FUN_00615ee0((LPCRITICAL_SECTION)(param_1 + 0xd));
  }
  if (param_1[0x14] == 0) {
    FUN_006171b0((int)param_1);
  }
  else {
    FUN_00616f90(param_1);
  }
  local_4 = 0xffffffff;
  FUN_00617dd0(param_1);
  ExceptionList = local_c;
  return;
}

