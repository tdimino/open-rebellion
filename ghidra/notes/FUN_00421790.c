
int * FUN_00421790(int *param_1,uint param_2,WPARAM param_3,int param_4)

{
  HWND hWnd;
  uint uVar1;
  int *this;
  void **ppvVar2;
  undefined4 uVar3;
  void *pvVar4;
  uint *puVar5;
  int iVar6;
  code *pcVar7;
  undefined2 extraout_var;
  undefined2 extraout_var_00;
  undefined2 extraout_var_01;
  UINT Msg;
  char *pcVar8;
  tagPAINTSTRUCT local_4c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062bac1;
  local_c = ExceptionList;
  Msg = 0;
  hWnd = (HWND)param_1[6];
  param_1 = (int *)0x0;
  if (param_2 < 0x10) {
    if (param_2 == 0xf) {
      ExceptionList = &local_c;
      BeginPaint(hWnd,&local_4c);
      EndPaint(hWnd,&local_4c);
    }
    else if (param_2 == 1) {
      ExceptionList = &local_c;
      this[9] = this[0x25];
      if (this[0x2a] != 0) {
        uVar3 = FUN_006037f0(0xc);
        pvVar4 = (void *)FUN_00618b70(0x28);
        local_4 = 0;
        if (pvVar4 == (void *)0x0) {
          puVar5 = (uint *)0x0;
        }
        else {
          puVar5 = (uint *)FUN_005fbd20(pvVar4,uVar3,0x2b35,2);
        }
        uVar1 = this[0xc];
        iVar6 = this[0xd];
        local_4 = 0xffffffff;
        pvVar4 = (void *)FUN_00618b70(0x28);
        local_4 = 1;
        if (pvVar4 == (void *)0x0) {
          pvVar4 = (void *)0x0;
        }
        else {
          pvVar4 = FUN_005fbda0(pvVar4,uVar1,iVar6,puVar5);
        }
        local_4 = 0xffffffff;
        this[0x2c] = (int)pvVar4;
        if (puVar5 != (uint *)0x0) {
          FUN_005fbfa0(puVar5);
          FUN_00618b60((undefined *)puVar5);
        }
        if ((void *)this[0x2c] != (void *)0x0) {
          FUN_005fcfe0((void *)this[0x2c],0,0,uVar1,iVar6,0);
        }
        pvVar4 = (void *)FUN_00618b70(0x178);
        local_4 = 2;
        if (pvVar4 == (void *)0x0) {
          pvVar4 = (void *)0x0;
        }
        else {
          pvVar4 = FUN_0041ecf0(pvVar4,uVar3,4,0x194,0x244,0x46,this,(uint *)this[0x2c],0);
        }
        local_4 = 0xffffffff;
        if (pvVar4 != (void *)0x0) {
          *(undefined4 *)((int)pvVar4 + 0x24) = 0xf2;
          *(undefined4 *)((int)pvVar4 + 0xb4) = 0x2ffffff;
          InvalidateRect(*(HWND *)((int)pvVar4 + 0x18),(RECT *)0x0,0);
          FUN_00420550(pvVar4,4);
          FUN_005f3090((void *)((int)pvVar4 + 0xa0),(int)(this + 0x29));
          *(undefined4 *)((int)pvVar4 + 0x13c) = 0;
          FUN_0041fc30(pvVar4);
          FUN_005ffce0(pvVar4,0);
          FUN_005f4f10(this + 0x1b,(int)pvVar4);
        }
      }
      pcVar8 = (char *)this[0x27];
      if (pcVar8 == (char *)0x0) {
        uVar3 = 0xff400;
        pcVar8 = (char *)this[0x28];
      }
      else {
        uVar3 = 0xfe400;
      }
      FUN_00421300(pcVar8,(void *)0x0,0,(int)this,1,(void *)0x1,uVar3);
      iVar6 = FUN_005f41b0(s_SOFTWARE_LucasArts_Entertainment_006a81fc,s_MusicVolume_006a8584);
      FUN_004213a0(-iVar6);
    }
  }
  else if (param_2 < 0x2c) {
    if (param_2 == 0x2b) {
      ExceptionList = &local_c;
      FUN_0059b100(this,param_4);
    }
    else if (param_2 == 0x14) {
      ExceptionList = &local_c;
      DefWindowProcA(hWnd,0x14,param_3,param_4);
      param_1 = (int *)0x1;
    }
  }
  else {
    if (param_2 < 0x112) {
      if (param_2 == 0x111) {
        if ((short)param_3 != 0xf1) {
          return (int *)0;
        }
        ExceptionList = &local_c;
        (**(code **)(*this + 8))();
        FUN_00421340();
        PostMessageA(*(HWND *)(this[8] + 0x18),0x405,this[0x25],this[0x26]);
        ExceptionList = local_c;
        return (int *)0;
      }
      if (param_2 != 0x100) {
        return (int *)0;
      }
      ppvVar2 = &local_c;
      if (param_3 != 0x1b) {
        if (this[0x26] == 0x71) {
          return (int *)0;
        }
        if (this[0x26] == 0x70) {
          return (int *)0;
        }
        ExceptionList = &local_c;
        (**(code **)(*this + 8))();
        FUN_00421340();
        Msg = 0x405;
        ppvVar2 = (void **)ExceptionList;
      }
      ExceptionList = ppvVar2;
      PostMessageA(*(HWND *)(this[8] + 0x18),Msg,this[0x25],this[0x26]);
      pcVar7 = (code *)this[0x2d];
      if (pcVar7 == (code *)0x0) {
        ExceptionList = local_c;
        return (int *)0;
      }
      iVar6 = this[0x26];
      uVar3 = CONCAT22(extraout_var_00,(short)this[0x25]);
    }
    else if (param_2 < 0x404) {
      if (param_2 < 0x402) {
        if (param_2 != 0x200) {
          return (int *)0;
        }
        ExceptionList = &local_c;
        SetCursor((HCURSOR)0x0);
        ExceptionList = local_c;
        return (int *)0;
      }
      ExceptionList = &local_c;
      (**(code **)(*this + 8))();
      FUN_00421340();
      PostMessageA(*(HWND *)(this[8] + 0x18),0x405,this[0x25],this[0x26]);
      pcVar7 = (code *)this[0x2d];
      if (pcVar7 == (code *)0x0) {
        ExceptionList = local_c;
        return (int *)0;
      }
      iVar6 = this[0x26];
      uVar3 = CONCAT22(extraout_var_01,(short)this[0x25]);
      Msg = 0;
    }
    else {
      if (param_2 != 0x404) {
        return (int *)0;
      }
      ExceptionList = &local_c;
      (**(code **)(*this + 8))();
      FUN_00421340();
      PostMessageA(*(HWND *)(this[8] + 0x18),0x405,this[0x25],this[0x26]);
      pcVar7 = (code *)this[0x2d];
      if (pcVar7 == (code *)0x0) {
        ExceptionList = local_c;
        return (int *)0;
      }
      iVar6 = this[0x26];
      uVar3 = CONCAT22(extraout_var,(short)this[0x25]);
      Msg = 0x405;
    }
    (*pcVar7)(Msg,uVar3,iVar6);
  }
  ExceptionList = local_c;
  return param_1;
}

