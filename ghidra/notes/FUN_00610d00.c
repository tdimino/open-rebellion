
byte __cdecl FUN_00610d00(undefined4 param_1,uint param_2,undefined4 param_3)

{
  byte bVar1;
  int iVar2;
  
  DAT_006be664 = param_1;
  if ((param_2 & 0x10000) != 0) {
    FUN_00611e90(param_1);
  }
  if (DAT_006be668 == (int *)0x0) {
    iVar2 = DirectSoundCreate(0,&DAT_006be668,0);
    if (iVar2 == 0) {
      iVar2 = (**(code **)(*DAT_006be668 + 0x18))(DAT_006be668,DAT_006be664,3);
      if (iVar2 == 0) {
        if ((param_2 & 0x100000) == 0) {
          bVar1 = FUN_00611330(0x2b110110);
        }
        else {
          bVar1 = FUN_00610e30(0x2b110208);
        }
      }
      else {
        bVar1 = 0xe;
      }
    }
    else {
      bVar1 = 6;
    }
  }
  else {
    bVar1 = 7;
  }
  if ((param_2 & 0x1000) != 0) {
    FUN_00611dc0(&DAT_006be608,param_1,param_3,DAT_006be668);
  }
  FUN_00611bf0();
  DAT_006be678 = DAT_006be678 + 1;
  return bVar1;
}

