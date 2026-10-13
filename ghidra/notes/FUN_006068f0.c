
void __fastcall FUN_006068f0(int *param_1)

{
  int iVar1;

  iVar1 = FUN_00607390(1);
  if (iVar1 != 0) {
    FUN_006010e0();
  }
  DAT_006be5ac = FUN_006037f0(param_1[0x25]);
  FUN_00607420();
  (**(code **)(*param_1 + 0x38))();
  (**(code **)(*param_1 + 0x3c))();
  (**(code **)(*param_1 + 0x40))();
  return;
}
