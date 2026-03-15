
undefined4 __cdecl FUN_0061b320(int *param_1)

{
  uint uVar1;
  uint uVar2;
  undefined4 uVar3;
  uint uVar4;
  
  uVar3 = 0;
  if ((((byte)param_1[3] & 3) == 2) && ((param_1[3] & 0x108U) != 0)) {
    uVar4 = *param_1 - param_1[2];
    if (0 < (int)uVar4) {
      uVar2 = FUN_0061b640(param_1[4],(char *)param_1[2],uVar4);
      uVar1 = param_1[3];
      if (uVar2 == uVar4) {
        if ((uVar1 & 0x80) != 0) {
          param_1[1] = 0;
          param_1[3] = uVar1 & 0xfffffffd;
          *param_1 = param_1[2];
          return 0;
        }
      }
      else {
        uVar3 = 0xffffffff;
        param_1[3] = uVar1 | 0x20;
      }
    }
  }
  param_1[1] = 0;
  *param_1 = param_1[2];
  return uVar3;
}

