
undefined4 __fastcall FUN_00485a50(int param_1)

{
  void *this;
  int iVar1;
  
  if (*(int *)(param_1 + 0x50) == 1) {
    iVar1 = FUN_004182a0((int *)(param_1 + 0x84));
    if (iVar1 != 0) {
      *(undefined4 *)(param_1 + 0x50) = 2;
    }
    return 0;
  }
  if (*(int *)(param_1 + 0x50) != 2) {
    *(undefined4 *)(param_1 + 0x50) = 1;
    return 0;
  }
  iVar1 = param_1 + 0x84;
  this = (void *)(param_1 + 100);
  FUN_004be930(this,0xd,iVar1);
  FUN_004be930(this,2,iVar1);
  FUN_004be930(this,1,iVar1);
  FUN_004be930(this,3,iVar1);
  FUN_004be930(this,6,iVar1);
  FUN_004be930(this,7,iVar1);
  FUN_004be930(this,4,iVar1);
  FUN_004be930(this,5,iVar1);
  FUN_004be930(this,8,iVar1);
  FUN_004be930(this,9,iVar1);
  FUN_004be930(this,10,iVar1);
  FUN_004be930(this,0xb,iVar1);
  FUN_004be930(this,0xc,iVar1);
  FUN_004be930(this,0xe,iVar1);
  *(undefined4 *)(param_1 + 0x428) = 10;
  *(undefined4 *)(param_1 + 0x50) = 4;
  return 1;
}

