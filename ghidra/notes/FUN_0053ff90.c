
uint __cdecl FUN_0053ff90(int param_1,int *param_2,void *param_3,int *param_4)

{
  int *this;
  int *piVar1;
  undefined4 *puVar2;
  int iVar3;
  uint *puVar4;
  uint uVar5;
  int iVar6;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  piVar1 = param_4;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00646a48;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_4 = 0;
  uVar5 = 1;
  if (DAT_006b90e0 != 0) {
    puVar2 = (undefined4 *)FUN_0054f100(param_1);
    *piVar1 = (int)puVar2;
    iVar6 = 0;
    if ((puVar2 != (undefined4 *)0x0) && (iVar6 = puVar2[6], iVar6 == 0)) {
      if (puVar2 != (undefined4 *)0x0) {
        (**(code **)*puVar2)(1);
      }
      *piVar1 = 0;
    }
    FUN_004ece30(&param_1);
    this = param_2;
    local_4 = 0;
    iVar3 = 0;
    if (iVar6 != 0) {
      iVar3 = FUN_00540450(param_2,&param_1);
    }
    FUN_004ece30(&param_4);
    local_4._0_1_ = 1;
    uVar5 = 0;
    if (iVar3 != 0) {
      uVar5 = (**(code **)(*this + 0xc))(&param_4);
    }
    iVar6 = *piVar1;
    puVar4 = FUN_004025b0(this,(uint *)&param_2);
    local_4._0_1_ = 2;
    FUN_004f26d0((void *)(iVar6 + 0x3c),puVar4);
    local_4._0_1_ = 1;
    FUN_00619730();
    FUN_004f26d0((void *)(iVar6 + 0x40),&param_1);
    FUN_004f26d0((void *)(iVar6 + 0x44),&param_4);
    FUN_004fd490((void *)(iVar6 + 0x20),param_3);
    local_4 = (uint)local_4._1_3_ << 8;
    *(uint *)(iVar6 + 0x48) = (uint)this[9] >> 4 & 3;
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  if ((uVar5 == 0) && ((undefined4 *)*piVar1 != (undefined4 *)0x0)) {
    (*(code *)**(undefined4 **)*piVar1)(1);
    *piVar1 = 0;
  }
  ExceptionList = pvStack_c;
  return uVar5;
}

