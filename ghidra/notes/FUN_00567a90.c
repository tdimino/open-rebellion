
uint __thiscall FUN_00567a90(void *this,int param_1,uint param_2,int param_3,int *param_4)

{
  int *piVar1;
  uint uVar2;
  void *this_00;
  int iVar3;
  bool bVar4;
  int local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  piVar1 = param_4;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064b240;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_4 = 0;
  FUN_0052d720(local_28 + 3);
  local_4 = 0;
  uVar2 = FUN_00567880(this,param_1,param_2,param_3,0,1,0,local_28 + 3,local_28);
  if (uVar2 != 0) {
    this_00 = (void *)thunk_FUN_005f5060((int)(local_28 + 3));
    if (this_00 != (void *)0x0) {
      FUN_0042ec10(this_00,&param_4);
      local_4 = CONCAT31(local_4._1_3_,1);
      local_28[1] = 0x10;
      local_28[2] = 0x40;
      if (((uint)param_4 >> 0x18 < 0x10) || (0x3f < (uint)param_4 >> 0x18)) {
        bVar4 = false;
      }
      else {
        bVar4 = true;
      }
      FUN_00619730();
      uVar2 = 0;
      if (bVar4) {
        iVar3 = FUN_00567790(this,(uint *)&param_4);
        *piVar1 = iVar3;
        uVar2 = (uint)(iVar3 != 0);
      }
      local_4 = local_4 & 0xffffff00;
      FUN_00619730();
    }
  }
  local_4 = 0xffffffff;
  FUN_0052d760(local_28 + 3);
  ExceptionList = local_c;
  return uVar2;
}

