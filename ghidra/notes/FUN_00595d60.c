
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl FUN_00595d60(undefined4 param_1,int param_2,undefined4 param_3)

{
  void *pvVar1;
  float in_stack_0000001c;
  float in_stack_00000020;
  float in_stack_00000024;
  undefined1 *in_stack_0000003c;
  undefined1 *in_stack_00000040;
  undefined1 *puStack00000044;
  undefined1 auStack_3c [4];
  float local_38;
  float local_34;
  float local_30;
  undefined1 auStack_24 [20];
  undefined4 uStack_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00650d28;
  local_c = ExceptionList;
  local_4 = 2;
  if (param_2 == 1) {
    puStack00000044 = auStack_24;
    ExceptionList = &local_c;
    FUN_004fcd90(auStack_24,(undefined4 *)&stack0x00000030);
    in_stack_00000040 = &stack0xffffffd8;
    local_4._0_1_ = 3;
    local_30 = 8.207004e-39;
    FUN_004fcd90(&stack0xffffffd8,(undefined4 *)&stack0x0000002c);
    local_34 = in_stack_00000020 * _DAT_0066babc;
    local_30 = in_stack_00000024;
    local_4._0_1_ = 4;
    local_38 = in_stack_0000001c * _DAT_0066babc;
    in_stack_0000003c = &stack0xffffffbc;
    FUN_005f2f90(&stack0xffffffbc,(int)&stack0x00000010);
    local_4 = CONCAT31(local_4._1_3_,2);
    pvVar1 = (void *)FUN_00597530();
    FUN_00599c90(pvVar1,param_1,param_3);
  }
  else {
    local_30 = in_stack_0000001c * _DAT_0066babc;
    puStack00000044 = auStack_3c;
    ExceptionList = &local_c;
    FUN_005f2f90(auStack_3c,(int)&stack0x00000010);
    local_4 = CONCAT31(local_4._1_3_,2);
    pvVar1 = (void *)FUN_00597530();
    FUN_0059a0b0(pvVar1,param_1,param_3);
  }
  local_4._0_1_ = 1;
  uStack_10 = 0x595e8f;
  FUN_005f2ff0((undefined4 *)&stack0x00000010);
  local_4 = (uint)local_4._1_3_ << 8;
  uStack_10 = 0x595e9d;
  FUN_00619730();
  local_4 = 0xffffffff;
  uStack_10 = 0x595eae;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

