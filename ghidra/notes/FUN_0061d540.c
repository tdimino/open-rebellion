
void FUN_0061d540(void)

{
  void *local_14;
  code *pcStack_10;
  undefined *puStack_c;
  undefined4 local_8;
  
  puStack_c = &DAT_0066e728;
  pcStack_10 = FUN_0061a414;
  local_14 = ExceptionList;
  ExceptionList = &local_14;
  if (PTR_FUN_006ad0a4 != (undefined *)0x0) {
    local_8 = 1;
    ExceptionList = &local_14;
    (*(code *)PTR_FUN_006ad0a4)();
  }
  local_8 = 0xffffffff;
  FUN_0061d5ae();
  ExceptionList = local_14;
  return;
}

