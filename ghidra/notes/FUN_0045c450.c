
void __thiscall FUN_0045c450(void *this,uint *param_1,int param_2,int param_3)

{
  bool bVar1;
  uint *puVar2;
  int *this_00;
  BOOL BVar3;
  char *pcVar4;
  int iVar5;
  uint uVar6;
  undefined4 auStack_30 [3];
  undefined4 auStack_24 [3];
  undefined4 auStack_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  puVar2 = param_1;
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_00630ad8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this_00 = (int *)FUN_00604500((void *)((int)this + 0x6c),(ushort)((short)*param_1 << 6) | 2);
  if (this_00 != (int *)0x0) {
    param_1 = (uint *)FUN_004f3220(*(int *)(*(int *)((int)this + 0x194) + 0x9c),puVar2);
    uVar6 = param_2;
    if ((param_1[0x14] & 8) == 0) {
      bVar1 = false;
      if ((this_00[0x25] == param_2) || (param_2 < 1)) {
        uVar6 = param_1[0x19];
      }
      else {
        SetWindowPos((HWND)this_00[6],(HWND)0x0,0,0,param_2 * 3,3,6);
        FUN_0060e400(this_00,uVar6,1);
        bVar1 = true;
      }
      iVar5 = param_3;
      if ((param_3 < 0) || (this_00[0x26] == param_3)) {
        iVar5 = this_00[0x26];
      }
      else {
        FUN_0060e440(this_00,param_3);
        bVar1 = true;
      }
      BVar3 = IsWindowVisible((HWND)this_00[6]);
      if ((BVar3 == 0) && ((param_1[0x22] & 2) != 0)) {
        (**(code **)(*this_00 + 4))(5);
        bVar1 = true;
      }
      if (bVar1) {
        param_1 = (uint *)CONCAT22(DAT_0065d424,0x1823);
        FUN_005f2fc0(auStack_30,&param_1);
        iStack_4 = 0;
        param_2 = CONCAT22(DAT_0065d424,0x1824);
        FUN_005f2fc0(auStack_18,&param_2);
        iStack_4._0_1_ = 1;
        param_3 = CONCAT22(DAT_0065d424,0x1825);
        FUN_005f2fc0(auStack_24,&param_3);
        iStack_4._0_1_ = 2;
        FUN_005f30d0(auStack_30,(int)auStack_18);
        FUN_005f31a0(auStack_30,iVar5);
        FUN_005f30d0(auStack_30,(int)auStack_24);
        FUN_005f31a0(auStack_30,uVar6);
        pcVar4 = (char *)FUN_00583c40((int)auStack_30);
        FUN_00600cc0(this,this_00,pcVar4);
        iStack_4._0_1_ = 1;
        FUN_005f2ff0(auStack_24);
        iStack_4 = (uint)iStack_4._1_3_ << 8;
        FUN_005f2ff0(auStack_18);
        iStack_4 = 0xffffffff;
        FUN_005f2ff0(auStack_30);
      }
    }
    else {
      (**(code **)(*this_00 + 8))();
    }
  }
  ExceptionList = local_c;
  return;
}

