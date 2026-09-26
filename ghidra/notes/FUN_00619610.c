
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float10 __cdecl FUN_00619610(uint param_1,uint param_2)

{
  double dVar1;
  uint uVar2;
  int iVar3;
  float10 fVar4;
  undefined4 local_8;
  undefined4 uStack_4;
  
  uVar2 = FUN_0061fee0();
  if ((param_2._2_2_ & 0x7ff0) != 0x7ff0) {
    fVar4 = FUN_0061f410((double)CONCAT44(param_2,param_1));
    dVar1 = (double)fVar4;
    if ((dVar1 != (double)CONCAT26(param_2._2_2_,CONCAT24((undefined2)param_2,param_1))) &&
       ((uVar2 & 0x20) == 0)) {
      uStack_4 = (undefined4)((ulonglong)dVar1 >> 0x20);
      local_8 = SUB84(dVar1,0);
      fVar4 = FUN_0061f480(0x10,0xb,param_1,param_2,local_8,uStack_4,uVar2);
      return fVar4;
    }
    FUN_0061fee0();
    return (float10)dVar1;
  }
  iVar3 = FUN_0061fd40(param_1,param_2);
  if (0 < iVar3) {
    if (iVar3 < 3) {
      FUN_0061fee0();
      return (float10)(double)CONCAT26(param_2._2_2_,CONCAT24((undefined2)param_2,param_1));
    }
    if (iVar3 == 3) {
      fVar4 = FUN_0061f430(0xb,param_1,param_2);
      return fVar4;
    }
  }
  dVar1 = (double)CONCAT26(param_2._2_2_,CONCAT24((undefined2)param_2,param_1)) - _DAT_0066e658;
  fVar4 = FUN_0061f480(8,0xb,param_1,param_2,SUB84(dVar1,0),(int)((ulonglong)dVar1 >> 0x20),uVar2);
  return fVar4;
}

