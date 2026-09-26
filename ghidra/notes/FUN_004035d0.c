
void * __thiscall
FUN_004035d0(void *this,uint param_1,uint param_2,uint param_3,uint param_4,uint param_5,
            uint param_6,int param_7,int param_8,void *param_9)

{
  uint uVar1;
  int iVar2;
  bool bVar3;
  void *this_00;
  void *pvVar4;
  undefined4 *puVar5;
  void *local_14;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062950d;
  local_c = ExceptionList;
  bVar3 = false;
  ExceptionList = &local_c;
  local_14 = (void *)FUN_00618b70(0x20);
  local_4 = 0;
  if (local_14 == (void *)0x0) {
    param_9 = (void *)0x0;
  }
  else {
    param_9 = FUN_0041be80(local_14,param_9);
  }
  local_4 = 0xffffffff;
  if (param_9 != (void *)0x0) {
    for (this_00 = (void *)thunk_FUN_005f5060((int)this); this_00 != (void *)0x0;
        this_00 = *(void **)((int)this_00 + 0x10)) {
      uVar1 = *(uint *)((int)this_00 + 0x30);
      if ((((((param_2 & uVar1) == param_2) && ((param_4 & uVar1) == 0)) &&
           ((param_3 & *(uint *)((int)this_00 + 0x34)) == param_3)) &&
          (((param_5 & *(uint *)((int)this_00 + 0x34)) == 0 &&
           ((param_1 < *(uint *)((int)this_00 + 0x2c) || ((uVar1 & 0x800000) != 0)))))) &&
         ((iVar2 = *(int *)((int)this_00 + (param_6 & 0xffff) * 4 + 0x40), param_7 <= iVar2 &&
          (iVar2 <= param_8)))) {
        pvVar4 = (void *)FUN_00618b70(0x18);
        local_4 = 1;
        local_10 = pvVar4;
        if (pvVar4 == (void *)0x0) {
          pvVar4 = (void *)0x0;
        }
        else {
          puVar5 = FUN_00403040(this_00,&local_14);
          bVar3 = true;
          local_4 = CONCAT31(local_4._1_3_,2);
          pvVar4 = FUN_0041c360(pvVar4,puVar5,iVar2);
        }
        local_4 = 1;
        if (bVar3) {
          bVar3 = false;
          FUN_00619730();
        }
        local_4 = 0xffffffff;
        FUN_0041c070(param_9,pvVar4);
      }
    }
    FUN_0041c230((int)param_9);
  }
  ExceptionList = local_c;
  return param_9;
}

