
void FUN_00509ef0(void)

{
  void *this;
  void *pvVar1;
  int unaff_EBX;
  int *unaff_ESI;
  int unaff_EDI;
  void *in_stack_00000018;
  
  if (unaff_EBX != unaff_EDI) {
    unaff_ESI[0x1b] = unaff_EDI;
    pvVar1 = in_stack_00000018;
    this = (void *)FUN_00539fd0(unaff_ESI,1);
    FUN_0050f590(this,pvVar1);
    pvVar1 = (void *)FUN_00539fd0(unaff_ESI,2);
    FUN_0050f590(pvVar1,in_stack_00000018);
    (**(code **)(*unaff_ESI + 0x1e4))();
  }
  return;
}

