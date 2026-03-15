
undefined4 __thiscall FUN_005f7ea0(void *this,int param_1)

{
  char cVar1;
  int iVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  DWORD DVar5;
  void *pvVar6;
  int iVar7;
  uint uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  char *pcVar11;
  undefined4 auStack_b0 [3];
  undefined4 auStack_a4 [3];
  undefined4 auStack_98 [3];
  undefined4 *local_8c;
  int local_88;
  void *local_84;
  undefined4 auStack_80 [3];
  char *local_74;
  undefined1 auStack_70 [100];
  void *local_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_006559fa;
  local_c = ExceptionList;
  uVar9 = 0;
  if (*(int *)((int)this + 0x108) == 0) {
    uVar9 = 1;
  }
  else if (*(uint *)((int)this + 0x94) < 2) {
    uVar9 = 0xb;
  }
  else {
    ExceptionList = &local_c;
    iVar2 = FUN_005f88c0((int)this);
    if (param_1 == iVar2) {
      uVar9 = 0xd;
    }
    else {
      pvVar6 = (void *)((int)this + param_1 * 0x20c);
      local_8c = (undefined4 *)((int)pvVar6 + 0x1b0);
      local_84 = pvVar6;
      cVar1 = FUN_005fb0c0((int)local_8c);
      if (cVar1 == '\0') {
        uVar9 = 0x10;
      }
      else if (*(int **)((int)this + 0x108) == (int *)0x0) {
        uVar9 = 1;
      }
      else {
        local_88 = **(int **)((int)this + 0x108);
        uVar8 = (uint)*(ushort *)((int)pvVar6 + 0x3b4);
        pcVar11 = (char *)((int)pvVar6 + 0x1b4);
        uVar10 = 0;
        local_74 = pcVar11;
        uVar3 = FUN_005f8ff0(this,param_1);
        iVar2 = FUN_005f88c0((int)this);
        uVar4 = FUN_005f8ff0(this,iVar2);
        iVar2 = (**(code **)(local_88 + 0x68))
                          (*(undefined4 *)((int)this + 0x108),uVar4,uVar3,uVar10,pcVar11,uVar8);
        if (iVar2 == 0) {
          (**(code **)*local_8c)();
        }
        else if (iVar2 == -0x7788fef2) {
          FUN_00618ed0(auStack_70,s_CommMgr__Flush___234_Error_Sendi_006ac330);
          FUN_005ae1e0();
          (**(code **)*local_8c)();
          uVar9 = 0;
        }
        else {
          FUN_005f2f50(auStack_98);
          iStack_4 = 0;
          FUN_005f35b0(auStack_b0,&DAT_006a8170);
          iStack_4._0_1_ = 1;
          FUN_005f35b0(auStack_a4,s_Error_Flushing_Packet__006ac318);
          iStack_4._0_1_ = 2;
          DVar5 = timeGetTime();
          pvVar6 = FUN_005f31f0(auStack_98,DVar5);
          FUN_005f30d0(pvVar6,(int)auStack_a4);
          FUN_005f30d0(pvVar6,(int)auStack_b0);
          iStack_4._0_1_ = 1;
          FUN_005f2ff0(auStack_a4);
          iStack_4._0_1_ = 0;
          FUN_005f2ff0(auStack_b0);
          FUN_005f35b0(auStack_a4,&DAT_006a8170);
          iStack_4._0_1_ = 3;
          FUN_005f35b0(auStack_b0,s_DPlay_Error_Code___006ac304);
          iStack_4._0_1_ = 4;
          FUN_005f30d0(auStack_98,(int)auStack_b0);
          pvVar6 = FUN_005f31f0(auStack_98,iVar2);
          FUN_005f30d0(pvVar6,(int)auStack_a4);
          iStack_4._0_1_ = 3;
          FUN_005f2ff0(auStack_b0);
          iStack_4._0_1_ = 0;
          FUN_005f2ff0(auStack_a4);
          FUN_005f35b0(auStack_a4,&DAT_006a8170);
          iStack_4._0_1_ = 5;
          FUN_005f35b0(auStack_b0,s_From___006ac2f8);
          iStack_4._0_1_ = 6;
          FUN_005f30d0(auStack_98,(int)auStack_b0);
          iVar7 = FUN_005f88c0((int)this);
          uVar9 = FUN_005f8ff0(this,iVar7);
          pvVar6 = FUN_005f31f0(auStack_98,uVar9);
          FUN_005f30d0(pvVar6,(int)auStack_a4);
          iStack_4._0_1_ = 5;
          FUN_005f2ff0(auStack_b0);
          iStack_4._0_1_ = 0;
          FUN_005f2ff0(auStack_a4);
          FUN_005f35b0(auStack_a4,&DAT_006a8170);
          iStack_4._0_1_ = 7;
          FUN_005f35b0(auStack_b0,s_To___006ac2ec);
          iStack_4._0_1_ = 8;
          FUN_005f30d0(auStack_98,(int)auStack_b0);
          uVar9 = FUN_005f8ff0(this,param_1);
          pvVar6 = FUN_005f31f0(auStack_98,uVar9);
          FUN_005f30d0(pvVar6,(int)auStack_a4);
          iStack_4._0_1_ = 7;
          FUN_005f2ff0(auStack_b0);
          iStack_4._0_1_ = 0;
          FUN_005f2ff0(auStack_a4);
          FUN_005f35b0(auStack_a4,&DAT_006a8170);
          iStack_4._0_1_ = 9;
          FUN_005f35b0(auStack_b0,s_Size___006ac2e0);
          iStack_4._0_1_ = 10;
          FUN_005f30d0(auStack_98,(int)auStack_b0);
          pvVar6 = FUN_005f31f0(auStack_98,(uint)*(ushort *)((int)local_84 + 0x3b4));
          FUN_005f30d0(pvVar6,(int)auStack_a4);
          iStack_4._0_1_ = 9;
          FUN_005f2ff0(auStack_b0);
          iStack_4._0_1_ = 0;
          FUN_005f2ff0(auStack_a4);
          FUN_005f35b0(auStack_80,&DAT_006a8170);
          pcVar11 = local_74;
          iStack_4._0_1_ = 0xb;
          FUN_005f35b0(auStack_a4,local_74);
          iStack_4._0_1_ = 0xc;
          FUN_005f35b0(auStack_b0,s_Message___006ac2d4);
          iStack_4._0_1_ = 0xd;
          FUN_005f30d0(auStack_98,(int)auStack_b0);
          FUN_005f30d0(auStack_98,(int)auStack_a4);
          FUN_005f30d0(auStack_98,(int)auStack_80);
          iStack_4._0_1_ = 0xc;
          FUN_005f2ff0(auStack_b0);
          iStack_4._0_1_ = 0xb;
          FUN_005f2ff0(auStack_a4);
          iStack_4._0_1_ = 0;
          FUN_005f2ff0(auStack_80);
          FUN_005f35b0(auStack_b0,&DAT_006a8170);
          iStack_4._0_1_ = 0xe;
          FUN_005f35b0(auStack_80,s_Address___006ac2c8);
          iStack_4._0_1_ = 0xf;
          FUN_005f30d0(auStack_98,(int)auStack_80);
          pvVar6 = FUN_005f31f0(auStack_98,pcVar11);
          FUN_005f30d0(pvVar6,(int)auStack_b0);
          iStack_4._0_1_ = 0xe;
          FUN_005f2ff0(auStack_80);
          iStack_4 = (uint)iStack_4._1_3_ << 8;
          FUN_005f2ff0(auStack_b0);
          FUN_00583c40((int)auStack_98);
          FUN_005ae1e0();
          if ((iVar2 == -0x7788ff6a) && (param_1 != 0xf)) {
            FUN_005ae1e0();
            PostMessageA(*(HWND *)((int)this + 0x2c),0x40c,4,param_1);
            uVar9 = 0xb;
          }
          else {
            uVar9 = 0xc;
          }
          PostMessageA(*(HWND *)((int)this + 0x2c),0x40c,4,param_1);
          FUN_005f8600((int *)this);
          iStack_4 = 0xffffffff;
          FUN_005f2ff0(auStack_98);
        }
      }
    }
  }
  ExceptionList = local_c;
  return uVar9;
}

