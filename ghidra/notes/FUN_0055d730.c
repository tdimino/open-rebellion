
uint __cdecl FUN_0055d730(undefined4 *param_1,void *param_2)

{
  int iVar1;
  uint uVar2;
  
  iVar1 = FUN_0053e2c0(1,100);
  iVar1 = FUN_0055bed0(3,iVar1);
  uVar2 = 0;
  if (iVar1 != 0) {
    *param_1 = *(undefined4 *)(iVar1 + 0x24);
    FUN_004f4000(param_2,(void *)(iVar1 + 0x28));
    uVar2 = *(uint *)((int)param_2 + 0xc);
  }
  return uVar2;
}

