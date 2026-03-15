
undefined4 __cdecl
FUN_00596fa0(undefined4 param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,
            void *param_5,undefined4 param_6,uint param_7,int param_8)

{
  void *this;
  undefined4 uVar1;
  
  uVar1 = 0x11;
  FUN_005f7d30();
  this = (void *)FUN_005f7cc0();
  FUN_005f4490(&DAT_006be370,param_5);
  if (this != (void *)0x0) {
    if (param_8 != 0) {
      uVar1 = FUN_005f8b80(this,param_6,param_1,param_2,param_3,param_4,param_7);
      return uVar1;
    }
    uVar1 = FUN_005ff810();
  }
  return uVar1;
}

