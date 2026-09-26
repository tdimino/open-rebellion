
void FUN_0050a670(void)

{
  int iVar1;
  void *pvVar2;
  int *unaff_ESI;
  int in_stack_00000010;
  void *in_stack_00000014;
  void *pvVar3;
  
  iVar1 = FUN_0053a640(0x40,in_stack_00000010,(uint *)(unaff_ESI + 0x22));
  if (iVar1 != 0) {
    pvVar3 = in_stack_00000014;
    pvVar2 = (void *)FUN_00539fd0(unaff_ESI,1);
    FUN_0050ff50(pvVar2,pvVar3);
    pvVar3 = in_stack_00000014;
    pvVar2 = (void *)FUN_00539fd0(unaff_ESI,2);
    FUN_0050ff50(pvVar2,pvVar3);
    (**(code **)(*unaff_ESI + 0x224))(in_stack_00000010,in_stack_00000014);
  }
  return;
}

