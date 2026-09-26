
float10 __cdecl FUN_0061fbb0(int param_1,int param_2)

{
  undefined *puVar1;
  int iVar2;
  undefined4 in_stack_0000001c;
  undefined4 in_stack_00000020;
  
  puVar1 = FUN_0061fc90(param_2);
  if (puVar1 != (undefined *)0x0) {
    FUN_0061fee0();
    iVar2 = FUN_006158b0();
    if (iVar2 == 0) {
      FUN_0061fc60(param_1);
    }
    return (float10)(double)CONCAT44(in_stack_00000020,in_stack_0000001c);
  }
  FUN_0061fee0();
  FUN_0061fc60(param_1);
  return (float10)(double)CONCAT44(in_stack_00000020,in_stack_0000001c);
}

