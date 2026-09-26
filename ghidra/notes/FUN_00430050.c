
void __thiscall FUN_00430050(void *this,uint *param_1,uint *param_2)

{
  bool bVar1;
  short sVar2;
  void *this_00;
  uint *puVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puVar3 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062cbb8;
  local_c = ExceptionList;
  if ((*param_1 >> 0x18 < 8) || (0xf < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  ExceptionList = &local_c;
  FUN_00619730();
  if (bVar1) {
    param_1 = (uint *)&stack0xffffffdc;
    FUN_004f26d0(&stack0xffffffdc,puVar3);
    this_00 = FUN_00419e40(this);
    if (this_00 != (void *)0x0) {
      FUN_004332b0(this_00,param_2);
      sVar2 = FUN_005f50e0((int)this_00 + 0x48);
      if (sVar2 == 0) {
        puVar3 = FUN_00403040(this_00,&param_1);
        local_4 = 0;
        FUN_00430220(this,puVar3);
        local_4 = 0xffffffff;
        FUN_00619730();
      }
    }
  }
  ExceptionList = local_c;
  return;
}

