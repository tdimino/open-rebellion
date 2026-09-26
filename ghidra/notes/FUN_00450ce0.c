
void __thiscall FUN_00450ce0(void *this,uint param_1,uint param_2,uint param_3,int param_4)

{
  bool bVar1;
  uint uVar2;
  void *pvVar3;
  int *piVar4;
  undefined4 uVar5;
  int iVar6;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  uVar2 = param_1;
  puStack_8 = &LAB_0062fbd8;
  local_c = ExceptionList;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00513120(param_1);
  iVar6 = *(int *)(uVar2 + 0x1c);
  do {
    if (iVar6 == 0) {
      local_4 = 0xffffffff;
      FUN_00619730();
      ExceptionList = local_c;
      return;
    }
    pvVar3 = (void *)FUN_0052bed0(uVar2);
    FUN_004025b0(pvVar3,&param_1);
    local_4 = CONCAT31(local_4._1_3_,2);
    if ((param_2 == 0) && (param_3 == 0)) {
      bVar1 = true;
    }
    else {
      bVar1 = false;
    }
    if (bVar1) {
LAB_00450d73:
      if ((param_2 == 0) && (param_3 == 0)) {
        bVar1 = true;
      }
      else {
        bVar1 = false;
      }
      FUN_00619730();
      if (bVar1) goto LAB_00450d9d;
      bVar1 = false;
    }
    else {
      if ((param_1 >> 0x18 < param_2) || (param_3 <= param_1 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      if (!bVar1) goto LAB_00450d73;
LAB_00450d9d:
      bVar1 = true;
    }
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
    if (bVar1) {
      pvVar3 = (void *)0x1;
      uVar5 = 4;
      iVar6 = param_4;
      piVar4 = (int *)FUN_0052bed0(uVar2);
      FUN_00450e10(this,piVar4,uVar5,iVar6,pvVar3);
    }
    local_4 = local_4 & 0xffffff00;
    FUN_00619730();
    FUN_005130d0(uVar2);
    iVar6 = *(int *)(uVar2 + 0x1c);
  } while( true );
}

