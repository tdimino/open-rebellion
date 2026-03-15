
int __thiscall FUN_004ed680(void *this,int param_1,undefined4 *param_2)

{
  undefined4 uVar1;
  int unaff_EBX;
  int iVar2;
  undefined4 *unaff_retaddr;
  void *local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = -1;
  puStack_8 = &LAB_0063ed98;
  pvStack_c = ExceptionList;
  iVar2 = 1;
  ExceptionList = &pvStack_c;
  *param_2 = 0;
  FUN_00520570(local_14);
                    /* WARNING: Load size is inaccurate */
  local_4 = 0;
  (**(code **)(*this + 0x2d0))(param_1,local_14);
  if (unaff_EBX == -1) {
    local_4 = 0;
    iVar2 = FUN_0053ebb0(*(undefined4 *)(param_1 + 0xc),&local_4);
    if (iVar2 != 0) {
      if ((*(short *)((int)this + 0x96) == local_4) && (*(short *)((int)this + 0x96) != 0)) {
        uVar1 = 1;
      }
      else {
        uVar1 = 0;
      }
      *unaff_retaddr = uVar1;
    }
  }
  pvStack_c = (void *)0xffffffff;
  FUN_00619730();
  ExceptionList = local_14[0];
  return iVar2;
}

